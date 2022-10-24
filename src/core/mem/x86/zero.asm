2:
    cmp {buf}, {end}
    jge 3f
    mov byte ptr [{buf}], 0
    inc {buf}
    jmp 2b
3:
