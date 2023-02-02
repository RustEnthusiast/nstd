    eor {len}, {len}, {len}
2:
    ldrb {byte:w}, [{cstr}]
    tst {byte}, {byte}
    beq 3f
    add {len}, {len}, 1
    add {cstr}, {cstr}, 1
    b 2b
3:
