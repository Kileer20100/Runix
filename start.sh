#!/bin/bash
cargo bootimage
nasm -f elf64 ./src/task/cpu_info/cpuinfo.asm -o ./src/task/cpu_info/cpu_info.o
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kernel/debug/bootimage-Runix.bin