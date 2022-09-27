2:
    mov {byte}, byte ptr [{src}]
    cmp {byte}, 0
    je 3f
    mov byte ptr [{dest}], {byte}
    inc {src}
    inc {dest}
    jmp 2b
3:
