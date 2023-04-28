.extern _malloc
.extern _calloc
.extern _realloc
.extern _free

.globl _nstd_os_unix_alloc_allocate
_nstd_os_unix_alloc_allocate:
    test rdi, rdi
    js 1f
    jmp _malloc
1:
    xor eax, eax
    ret

.globl _nstd_os_unix_alloc_allocate_zeroed
_nstd_os_unix_alloc_allocate_zeroed:
    test rdi, rdi
    js 2f
    mov esi, 1
    jmp _calloc
2:
    xor eax, eax
    ret

.globl _nstd_os_unix_alloc_reallocate
_nstd_os_unix_alloc_reallocate:
    test rsi, rsi
    js 3f
    push rdi
    mov rdi, [rdi]
    call _realloc
    pop rdi
    test rax, rax
    jz 4f
    mov [rdi], rax
    xor eax, eax
    ret
3:
    mov eax, 2
    ret
4:
    mov eax, 1
    ret

.globl _nstd_os_unix_alloc_deallocate
_nstd_os_unix_alloc_deallocate:
    push rdi
    mov rdi, [rdi]
    call _free
    pop rdi
    mov qword ptr [rdi], 0
    ret
