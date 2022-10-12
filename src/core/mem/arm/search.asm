    mov {end}, {buf}
    add {end}, {size}
2:
    cmp {buf}, {end}
    bge 3f
    ldrb {byte}, [{buf}]
    cmp {byte}, {delim}
    beq 4f
    add {buf}, 1
    bal 2b
3:
    eor {buf}, {buf}
4:
