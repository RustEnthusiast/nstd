2:
    ldrb {byte}, [{src}]
    tst {byte}, {byte}
    beq 3f
    strb {byte}, [{dest}]
    add {src}, 1
    add {dest}, 1
    bal 2b
3:
