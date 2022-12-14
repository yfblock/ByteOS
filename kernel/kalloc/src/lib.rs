#![no_std]
#![feature(used_with_arg)]

// 这个模块暂时还是借用已有的实现，后面如果有时间可以再进行修改

use buddy_system_allocator::LockedHeap;

// 堆大小
const HEAP_SIZE: usize = 0x0008_0000;

// 堆空间
static mut HEAP: [u8;HEAP_SIZE] = [0;HEAP_SIZE];

// 堆内存分配器
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<64> = LockedHeap::empty();

// 初始化堆内存分配器
#[header::distributed_slice(header::INIT_FUNC_PRIOR_0)]
pub fn init() {
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, HEAP_SIZE);
    }
}