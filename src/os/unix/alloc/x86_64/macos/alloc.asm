.extern _malloc
.extern _calloc
.extern _realloc
.extern _free

.globl _nstd_os_unix_alloc_allocate
_nstd_os_unix_alloc_allocate:
    jmp _malloc

.globl _nstd_os_unix_alloc_allocate_zeroed
_nstd_os_unix_alloc_allocate_zeroed:
    mov esi, 1
    jmp _calloc

.globl _nstd_os_unix_alloc_allocate
_nstd_os_unix_alloc_allocate:
    push rdi
    mov rdi, [rdi]
    call _realloc
    pop rdi
    test rax, rax
    jz 1f
    mov [rdi], rax
    xor eax, eax
    ret
1:
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
