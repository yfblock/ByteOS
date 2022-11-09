// remove std lib
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(asm_sym)]
#![allow(unaligned_references)]
#![feature(used_with_arg)]

extern crate alloc;

#[macro_use]
extern crate console;
mod panic;
mod module;

use core::arch::asm;

use alloc::boxed::Box;
use header::Fdt;

/// 汇编入口函数
/// 
/// 分配栈 并调到rust入口函数
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    const STACK_SIZE: usize = 4096;

    #[link_section = ".bss.stack"]
    static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    core::arch::asm!(
        "   la  sp, {stack} + {stack_size}
            j   rust_main
        ",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        options(noreturn),
    )
}

/// rust 入口函数
/// 
/// 进行操作系统的初始化，
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, device_tree_addr: usize) -> ! {
    // 保存设备树信息
    let fdt = header::DEVICE_TREE_ADDR.call_once(|| 
        unsafe { Fdt::from_ptr(device_tree_addr as _).unwrap() }
    );

    println!(r"     ____        __          ____  _____");
    println!(r"    / __ )__  __/ /____     / __ \/ ___/");
    println!(r"   / __  / / / / __/ _ \   / / / /\__ \ ");
    println!(r"  / /_/ / /_/ / /_/  __/  / /_/ /___/ / ");
    println!(r" /_____/\__, /\__/\___/   \____//____/  ");
    println!(r"       /____/                           "); 

    println!("[kernel] welcome to use ByteOS");

    // 输出设备信息
    println!("[kernel] device tree addr @ 0x{:X}", device_tree_addr);
    println!("[kernel] {} cpus", fdt.cpus().count());

    for i in fdt.memory().regions() {
        println!("[kernel] memory region @ {:?}  size: {:X?}", i.starting_address, i.size);
    }

    println!("[kernel] default console uart: {:?}", fdt.chosen().stdout().unwrap().name);

    // 执行优先级最高的初始化函数
    header::INIT_FUNC_PRIOR_0.iter().for_each(|f| f());

    // 执行优先级1的初始化函数
    header::INIT_FUNC_PRIOR_1.iter().for_each(|f| f());

    // 执行优先级2的初始化函数
    header::INIT_FUNC_PRIOR_2.iter().for_each(|f| f());

    // 执行优先级3的初始化函数
    header::INIT_FUNC_PRIOR_3.iter().for_each(|f| f());

    // 让其他核心进入等待 用在多核的情况 单核下无需使用。
    // if hart_id != 0 {
    //     support_hart_resume(hart_id, 0);
    // }

    // 测试 Allocator
    let test1 = Box::new("123");
    println!("{}", test1);


    
    // 调用rust api关机
    panic!("正常关机")
}


/// 辅助核心进入的函数
/// 
/// 目前让除 0 核之外的其他内核进入该函数进行等待
#[allow(unused)]
extern "C" fn support_hart_resume(hart_id: usize, _param: usize) {
    loop {
        // 使用wfi 省电
        unsafe { asm!("wfi") }
    }
}
