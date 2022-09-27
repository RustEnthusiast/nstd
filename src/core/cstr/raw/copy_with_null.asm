2:
    mov {byte}, byte ptr [{src}]
    mov byte ptr [{dest}], {byte}
    cmp {byte}, 0
    je 3f
    inc {src}
    inc {dest}
    jmp 2b
3:
