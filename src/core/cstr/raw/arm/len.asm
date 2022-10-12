    eor {i}, {i}
2:
    ldrb {byte}, [{cstr}, {i}]
    cmp {byte}, 0
    beq 3f
    add {i}, 1
    bal 2b
3:
