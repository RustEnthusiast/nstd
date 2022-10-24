    eor {zero}, {zero}, {zero}
2:
    cmp {buf}, {end}
    bge 3f
    strb {zero:w}, [{buf}]
    add {buf}, {buf}, 1
    b 2b
3:
