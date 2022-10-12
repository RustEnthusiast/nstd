    eor {is_nt}, {is_nt}
    eor {i}, {i}
2:
    cmp {i}, {len}
    bge 4f
    ldrb {byte}, [{ptr}, {i}]
    cmp {byte}, 0
    beq 3f
    add {i}, 1
    bal 2b
3:
    add {i}, 1
    cmp {i}, {len}
    bne 4f
    add {is_nt}, 1
4:
