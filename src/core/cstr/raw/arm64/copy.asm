2:
    ldrb {byte:w}, [{src}]
    tst {byte}, {byte}
    beq 3f
    strb {byte:w}, [{dest}]
    add {dest}, {dest}, 1
    add {src}, {src}, 1
    b 2b
3:
