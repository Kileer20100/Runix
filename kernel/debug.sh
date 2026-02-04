cargo bootimage
nasm -f elf64 src/cpu/cpu_info.asm -o cpu_info.o
ar rcs libasm.a cpu_info.o