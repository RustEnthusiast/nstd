    mov {is_eq}, 1
    cmp {cstr1}, {cstr2}
    je 4f
2:
    mov {byte}, byte ptr [{cstr1}]
    cmp {byte}, byte ptr [{cstr2}]
    jne 3f
    test {byte}, {byte}
    jz 4f
    inc {cstr1}
    inc {cstr2}
    jmp 2b
3:
    xor {is_eq}, {is_eq}
4:
