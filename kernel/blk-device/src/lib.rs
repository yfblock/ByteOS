#![no_std]
#![feature(used_with_arg)]

use core::ptr::NonNull;

use console::println;
use fdt::Fdt;
use header::distributed_slice;
use header::INIT_FUNC_PRIOR_2;
use virtio_drivers::DeviceType;
use virtio_drivers::MmioTransport;
use virtio_drivers::Transport;
use virtio_drivers::VirtIOHeader;

use dev::MMIO_ARR;

extern crate alloc;

// TODO: 创建一个列表 把适合的设备进行存储。 然后在需要的地方打开 需要实现 device type 的 HAL interface

#[distributed_slice(INIT_FUNC_PRIOR_2)]
fn read_block_device() {
    MMIO_ARR.wait().iter().for_each(|device| {
        if let DeviceType::Block = device.device_type() {
            println!("[kernel] BLOCK DEVICE: {:?}", device);
        }
    });
}