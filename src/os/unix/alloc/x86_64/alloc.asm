.extern malloc
.extern calloc
.extern realloc
.extern free

.globl nstd_os_unix_alloc_allocate
nstd_os_unix_alloc_allocate:
    test rdi, rdi
    js 1f
    jmp malloc
1:
    xor eax, eax
    ret

.globl nstd_os_unix_alloc_allocate_zeroed
nstd_os_unix_alloc_allocate_zeroed:
    test rdi, rdi
    js 2f
    mov esi, 1
    jmp calloc
2:
    xor eax, eax
    ret

.globl nstd_os_unix_alloc_reallocate
nstd_os_unix_alloc_reallocate:
    test rsi, rsi
    js 3f
    push rdi
    mov rdi, [rdi]
    call realloc
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

.globl nstd_os_unix_alloc_deallocate
nstd_os_unix_alloc_deallocate:
    push rdi
    mov rdi, [rdi]
    call free
    pop rdi
    mov qword ptr [rdi], 0
    ret
