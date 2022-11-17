2:
    cmp {buf}, {end}
    jge 3f
    cmp byte ptr [{buf}], {delim}
    je 4f
    inc {buf}
    jmp 2b
3:
    xor {buf}, {buf}
4:
