global get_info_cpu

section .text
get_info_cpu:
    ; rdi = номер функции CPUID
    ; rsi = ptr для eax
    ; rdx = ptr для ebx
    ; rcx = ptr для ecx
    ; r8  = ptr для edx

    mov eax, edi
    cpuid

    mov [rsi], eax
    mov [rdx], ebx
    mov [rcx], ecx
    mov [r8], edx
    ret
