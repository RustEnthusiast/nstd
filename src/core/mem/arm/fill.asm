2:
    cmp {buf}, {end}
    bge 3f
    strb {fill}, [{buf}]
    add {buf}, {buf}, 1
    b 2b
3:
