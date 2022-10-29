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
mod console;
mod panic;
mod sbi;
mod module;

use core::arch::asm;

use alloc::boxed::Box;

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
pub extern "C" fn rust_main(_hart_id: usize, _device_tree_addr: usize) -> ! {

    // 让其他核心进入等待 用在多核的情况 单核下无需使用。
    // if hart_id != 0 {
    //     support_hart_resume(hart_id, 0);
    // }

    println!("[kernel] welcome to use ByteOS");

    // 执行优先级最高的初始化函数
    header::INIT_FUNC_PRIOR_0.iter().for_each(|f| f());

    // 测试 Allocator
    let test1 = Box::new("123");
    println!("{}", test1);

    // 测试设备树代码
    // use dtb_walker::{utils::indent, Dtb, DtbObj, HeaderError as E, WalkOperation as Op};

    // const INDENT_WIDTH: usize = 4;

    // let dtb = unsafe {
    //     Dtb::from_raw_parts_filtered(_device_tree_addr as _, |e| {
    //         matches!(e, E::Misaligned(4) | E::LastCompVersion(16))
    //     })
    // }
    // .map_err(|e| format!("verify header failed: {e:?}")).expect("header error");
    // dtb.walk(|path, obj| match obj {
    //     DtbObj::SubNode { name } => {
    //         // println!("{}{}/{:?}", indent(path.level(), INDENT_WIDTH), path, name);
    //         Op::StepInto
    //     }
    //     DtbObj::Property(prop) => {
    //         if !path.to_string().starts_with("/memory") { return Op::StepOver; }
    //         let indent = indent(path.level(), INDENT_WIDTH);
    //         println!("{}{:?}", indent, prop);
    //         Op::StepOver
    //     }
    // });
    
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
