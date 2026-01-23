#!/bin/bash
cargo bootimage
nasm -f elf64 src/cpu/cpu_info.asm -o cpu_info.o
ar rcs libasm.a cpu_info.o
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kernel/debug/bootimage-Runix.bin