#![no_std]
#![feature(used_with_arg)]
// use header::linkme as linkme;
use linkme;

#[linkme::distributed_slice(task::TASKS)]
pub fn test_func() -> usize {
    123
}

#[linkme::distributed_slice(task::TASKS)]
pub fn func1() -> usize {
    456
}

#[linkme::distributed_slice(task::TASKS)]
pub fn func2() -> usize {
    789
}