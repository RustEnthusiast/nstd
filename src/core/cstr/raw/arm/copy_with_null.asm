2:
    ldrb {byte}, [{src}]
    strb {byte}, [{dest}]
    tst {byte}, {byte}
    beq 3f
    add {src}, 1
    add {dest}, 1
    bal 2b
3:
