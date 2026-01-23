#!/bin/bash
cargo bootimage
<<<<<<< HEAD
nasm -f elf64 src/cpu/cpu_info.asm -o cpu_info.o
ar rcs libasm.a cpu_info.o
=======
<<<<<<< HEAD
#nasm -f elf64 ./src/task/cpu_info/cpuinfo.asm -o ./src/task/cpu_info/cpu_info.o
=======
nasm -f elf64 src/cpu/cpu_info.asm -o cpu_info.o
ar rcs libasm.a cpu_info.o
>>>>>>> 3ba3fc5 (connection ASM)
>>>>>>> main
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kernel/debug/bootimage-Runix.bin