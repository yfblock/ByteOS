#![no_std]
#![feature(used_with_arg)]

use linkme::distributed_slice;

#[distributed_slice]
pub static TASKS: [fn () -> usize] = [..];

#[distributed_slice]
pub static SHENANIGANS: [usize] = [..];

#[distributed_slice(SHENANIGANS)]
static N: usize = 9;


pub fn test() -> usize {
    let mut n = 0;
    for f in TASKS {
        n = n + f();
    }
    n
}