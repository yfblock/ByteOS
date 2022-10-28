#![no_std]
#![feature(used_with_arg)]

pub use linkme;
pub use macros::byteos_module_use;
pub use macros::distributed_slice;

#[linkme::distributed_slice]
pub static INIT_FUNC_PRIOR_0: [fn()] = [..];