2:
    cmp {buf}, {end}
    jge 3f
    mov byte ptr [{buf}], {fill}
    inc {buf}
    jmp 2b
3:
