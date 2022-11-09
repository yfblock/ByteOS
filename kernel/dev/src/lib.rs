#![no_std]
#![feature(used_with_arg)]

use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::NonNull;

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use console::println;
use header::distributed_slice;
use header::INIT_FUNC_PRIOR_1;
use spin::Once;
use virtio_drivers::MmioTransport;
use virtio_drivers::Transport;
use virtio_drivers::VirtIOHeader;

// use spin::lazy::Lazy;
// use spin::once::Once;

pub struct MmioVec(Vec<MmioTransport>);

impl Deref for MmioVec {
    type Target = Vec<MmioTransport>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MmioVec {
    // type Target = Vec<MmioTransport>;

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MmioVec {
    pub const fn new() -> Self {
        Self(vec![])
    }
}

unsafe impl Sync for MmioVec{}
unsafe impl Send for MmioVec{}

pub static MMIO_ARR: Once<MmioVec> = Once::new();

#[distributed_slice(INIT_FUNC_PRIOR_1)]
pub fn init() {
    // 初始化内存
    let fdt = header::DEVICE_TREE_ADDR.wait();
    let mut mmio_vec = MmioVec::new();
    for node in fdt.all_nodes() {
        if let Some(compatible) = node.compatible() {
            if compatible.all().any(|s| s == "virtio,mmio") {
                if let Some(reg) = node.reg().and_then(|mut reg| reg.next()) {
                    let paddr = reg.starting_address as usize;
                    let vaddr = paddr;
                    let header = NonNull::new(vaddr as *mut VirtIOHeader).unwrap();
                    match unsafe { MmioTransport::new(header) } {
                        Err(_e) => {
                            // println!("[MMIO] Error creating VirtIO MMIO transport: {}", e)
                        },
                        Ok(transport) => {
                            println!(
                                "[device] Detected virtio MMIO device with vendor id {:#X}, device type {:?}, version {:?}",
                                transport.vendor_id(),
                                transport.device_type(),
                                transport.version(),
                            );
                            // 保存 MMIO
                            mmio_vec.push(transport);
                        }
                    }
                }
            }
        }
    }
    
    // 存储 MMIO 设备列表
    MMIO_ARR.call_once(|| mmio_vec);

}