.PHONY: all run clean

TARGET      := riscv32imac-unknown-none-elf
KERNEL_FILE := target/$(TARGET)/release/os
DEBUG_FILE  ?= $(KERNEL_FILE)

OBJDUMP     := rust-objdump --arch-name=riscv32
OBJCOPY     := rust-objcopy --binary-architecture=riscv32

all:
	cargo build --release
	cp $(KERNEL_FILE) kernel-qemu

run: all
	qemu-system-riscv32 \
    -machine virt \
    -bios default \
    -device loader,file=kernel-qemu,addr=0x80400000 \
    -nographic \
    -smp 4 -m 64m

clean:
	rm kernel-qemu
	rm $(KERNEL_FILE)