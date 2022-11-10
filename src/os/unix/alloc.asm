.extern malloc
.extern calloc
.extern realloc
.extern free

.globl nstd_os_unix_alloc_allocate
nstd_os_unix_alloc_allocate:
    sub rsp, 8
    call malloc
    add rsp, 8
    ret

.globl nstd_os_unix_alloc_allocate_zeroed
nstd_os_unix_alloc_allocate_zeroed:
    sub rsp, 8
    mov rsi, 1
    call calloc
    add rsp, 8
    ret

.globl nstd_os_unix_alloc_reallocate
nstd_os_unix_alloc_reallocate:
    push rbx
    mov rbx, rdi
    mov rdi, [rdi]
    call realloc
    test rax, rax
    jz 1f
    mov [rbx], rax
    xor rax, rax
    pop rbx
    ret
1:
    mov rax, 1
    pop rbx
    ret

.globl nstd_os_unix_alloc_deallocate
nstd_os_unix_alloc_deallocate:
    push rdi
    mov rdi, [rdi]
    call free
    pop rdi
    mov qword ptr [rdi], 0
    ret
