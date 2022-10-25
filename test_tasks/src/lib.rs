#![no_std]
#![feature(used_with_arg)]
// use header::linkme as linkme;
use linkme;

#[linkme::distributed_slice(task::TASKS)]
pub fn test_func() -> usize {
    123
}

pub fn test() {

}