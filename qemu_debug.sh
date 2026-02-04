qemu-system-x86_64 \
    -cdrom Runix.iso \
    -chardev stdio,id=seabios \
    -device isa-debugcon,iobase=0xe9,chardev=seabios