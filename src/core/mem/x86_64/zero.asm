2:
    cmp {buf}, {reg_end}
    jge 3f
    mov qword ptr [{buf}], 0
    add {buf}, 8
    jmp 2b
3:
    cmp {buf}, {end}
    jge 4f
    mov byte ptr [{buf}], 0
    inc {buf}
    jmp 3b
4: