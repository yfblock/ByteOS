#![no_std]
#![feature(used_with_arg)]

use core::ptr::NonNull;

use console::println;
use fdt::Fdt;
use header::distributed_slice;
use header::INIT_FUNC_PRIOR_0;
use virtio_drivers::DeviceType;
use virtio_drivers::MmioTransport;
use virtio_drivers::Transport;
use virtio_drivers::VirtIOHeader;

// #[distributed_slice]
// pub static BLOCK_DEVICES

#[distributed_slice(INIT_FUNC_PRIOR_0)]
fn read_block_device() {
    let fdt = unsafe { Fdt::from_ptr(*(header::DEVICE_TREE_ADDR.get().unwrap()) as *const u8).unwrap() };
    for node in fdt.all_nodes() {
        if let Some(compatible) = node.compatible() {
            if compatible.all().any(|s| s == "virtio,mmio") {
                if let Some(reg) = node.reg().and_then(|mut reg| reg.next()) {
                    let paddr = reg.starting_address as usize;
                    let size = reg.size.unwrap();
                    let vaddr = paddr;
                    let header = NonNull::new(vaddr as *mut VirtIOHeader).unwrap();
                    match unsafe { MmioTransport::new(header) } {
                        Err(e) => println!("Error creating VirtIO MMIO transport: {}", e),
                        Ok(transport) => {
                            println!(
                                "Detected virtio MMIO device with vendor id {:#X}, device type {:?}, version {:?}",
                                transport.vendor_id(),
                                transport.device_type(),
                                transport.version(),
                            );
                            match transport.device_type() {
                                DeviceType::Block => {
                                    println!("device");
                                },
                                t => println!("Unrecognized virtio device: {:?}", t),
                            }
                        }
                    }
                }
            }
        }
    }
}