2:
    mov {byte}, byte ptr [{src}]
    test {byte}, {byte}
    je 3f
    mov byte ptr [{dest}], {byte}
    inc {src}
    inc {dest}
    jmp 2b
3:
