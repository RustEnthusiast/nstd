2:
    cmp {buf}, {end}
    bge 3f
    strb {fill:w}, [{buf}]
    add {buf}, {buf}, 1
    b 2b
3:
