2:
    mov {byte}, byte ptr [{src}]
    mov byte ptr [{dest}], {byte}
    test {byte}, {byte}
    je 3f
    inc {src}
    inc {dest}
    jmp 2b
3:
