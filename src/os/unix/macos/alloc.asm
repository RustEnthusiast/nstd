.extern malloc
.extern calloc
.extern realloc
.extern free

.globl _nstd_os_unix_alloc_allocate
_nstd_os_unix_alloc_allocate:
    sub rsp, 8
    call _malloc
    add rsp, 8
    ret

.globl _nstd_os_unix_alloc_allocate_zeroed
_nstd_os_unix_alloc_allocate_zeroed:
    sub rsp, 8
    mov rsi, 1
    call _calloc
    add rsp, 8
    ret

.globl _nstd_os_unix_alloc_reallocate
_nstd_os_unix_alloc_reallocate:
    push rbx
    mov rbx, rdi
    mov rdi, [rdi]
    call _realloc
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

.globl _nstd_os_unix_alloc_deallocate
_nstd_os_unix_alloc_deallocate:
    push rdi
    mov rdi, [rdi]
    call _free
    pop rdi
    mov qword ptr [rdi], 0
    ret
