2:
    cmp {buf}, {end}
    bge 3f
    ldrb {byte}, [{buf}]
    cmp {byte}, {delim}
    beq 4f
    add {buf}, {buf}, 1
    b 2b
3:
    eor {buf}, {buf}, {buf}
4:
