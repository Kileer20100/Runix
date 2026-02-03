; src/cpu/cpu_info add
bits 64
global add

section .text
add:
    mov rax, rdi
    add rax, rsi
    ret