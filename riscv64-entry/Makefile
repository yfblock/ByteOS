.PHONY: all run clean

TARGET      := riscv64imac-unknown-none-elf
KERNEL_FILE := target/$(TARGET)/release/os
DEBUG_FILE  ?= $(KERNEL_FILE)

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

all:
	cargo build --release
	cp $(KERNEL_FILE) kernel-qemu

run: all
	qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -kernel kernel-qemu \
    -nographic \
    -smp 4 -m 64m

clean:
	rm kernel-qemu
	rm $(KERNEL_FILE)