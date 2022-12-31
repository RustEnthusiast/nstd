2:
    mov {byte}, byte ptr [{src}]
    mov byte ptr [{dest}], {byte}
    test {byte}, {byte}
    jz 3f
    inc {dest}
    inc {src}
    jmp 2b
3:
