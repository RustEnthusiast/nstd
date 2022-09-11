    xor {is_nt}, {is_nt}
    xor {i}, {i}
2:
    cmp {i}, {len}
    jge 4f
    cmp byte ptr [{ptr} + {i}], 0
    je 3f
    inc {i}
    jmp 2b
3:
    inc {i}
    cmp {i}, {len}
    jne 4f
    mov {is_nt}, 1
4:
