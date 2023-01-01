2:
    ldrb {byte:w}, [{src}]
    strb {byte:w}, [{dest}]
    tst {byte}, {byte}
    beq 3f
    add {dest}, {dest}, 1
    add {src}, {src}, 1
    b 2b
3:
