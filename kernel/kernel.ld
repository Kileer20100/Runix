/* Явно указываем что это статический исполняемый файл */
OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

/* Limine загружает по 1MB */
PHDRS {
    text PT_LOAD FLAGS((1 << 0) | (1 << 2)); /* RX */
    rodata PT_LOAD FLAGS((1 << 2));          /* R */
    data PT_LOAD FLAGS((1 << 1) | (1 << 2)); /* RW */
    bss PT_LOAD FLAGS((1 << 1) | (1 << 2));  /* RW */
}

SECTIONS {
    . = 1M;
    
    .text : ALIGN(4K) {
        *(.text .text.*)
    } :text
    
    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    } :rodata
    
    .data : ALIGN(4K) {
        *(.data .data.*)
    } :data
    
    .bss : ALIGN(4K) {
        *(COMMON)
        *(.bss .bss.*)
    } :bss
    
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.*)
    }
}