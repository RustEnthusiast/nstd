.extern malloc
.extern calloc
.extern realloc
.extern free

.globl nstd_os_unix_alloc_allocate
nstd_os_unix_alloc_allocate:
    jmp malloc

.globl nstd_os_unix_alloc_allocate_zeroed
nstd_os_unix_alloc_allocate_zeroed:
    mov esi, 1
    jmp calloc

.globl nstd_os_unix_alloc_reallocate
nstd_os_unix_alloc_reallocate:
    push rdi
    mov rdi, [rdi]
    call realloc
    pop rdi
    test rax, rax
    jz 1f
    mov [rdi], rax
    xor eax, eax
    ret
1:
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
