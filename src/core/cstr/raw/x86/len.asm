    xor {len}, {len}
2:
    cmp byte ptr [{cstr}], 0
    je 3f
    inc {len}
    inc {cstr}
    jmp 2b
3:
