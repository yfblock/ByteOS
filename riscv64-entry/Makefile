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
    -smp 4 -m 128m

clean:
	rm kernel-qemu
	rm $(KERNEL_FILE)