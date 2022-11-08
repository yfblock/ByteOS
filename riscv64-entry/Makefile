.PHONY: all run clean

TARGET      := riscv64imac-unknown-none-elf
KERNEL_FILE := ../target/$(TARGET)/release/os
DEBUG_FILE  ?= $(KERNEL_FILE)

all:
	cargo build --release
	cp $(KERNEL_FILE) kernel-qemu

run: all
	qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -kernel kernel-qemu \
    -nographic \
	-drive file=fat32.img,if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
    -smp 4 -m 128m

clean:
	rm kernel-qemu
	rm $(KERNEL_FILE)