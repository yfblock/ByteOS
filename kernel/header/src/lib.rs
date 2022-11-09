#![no_std]
#![feature(used_with_arg)]

pub use fdt::Fdt;
pub use linkme;
pub use macros::byteos_module_use;
pub use macros::distributed_slice;
use spin::Once;

#[linkme::distributed_slice]
pub static INIT_FUNC_PRIOR_0: [fn()] = [..];
#[linkme::distributed_slice]
pub static INIT_FUNC_PRIOR_1: [fn()] = [..];
#[linkme::distributed_slice]
pub static INIT_FUNC_PRIOR_2: [fn()] = [..];
#[linkme::distributed_slice]
pub static INIT_FUNC_PRIOR_3: [fn()] = [..];
/// 设备树地址
pub static DEVICE_TREE_ADDR: Once<Fdt> = Once::new();