2:
    mov {byte}, byte ptr [{src}]
    test {byte}, {byte}
    jz 3f
    mov byte ptr [{dest}], {byte}
    inc {dest}
    inc {src}
    jmp 2b
3:
