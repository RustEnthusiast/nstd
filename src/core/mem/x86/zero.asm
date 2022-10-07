    xor {i}, {i}
2:
    cmp {i}, {size}
    jge 3f
    mov byte ptr [{buf} + {i}], 0
    inc {i}
    jmp 2b
3:
