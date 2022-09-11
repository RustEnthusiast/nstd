    xor {i}, {i}
2:
    cmp byte ptr [{cstr} + {i}], 0
    je 3f
    inc {i}
    jmp 2b
3:
