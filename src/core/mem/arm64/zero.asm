    eor {zero}, {zero}, {zero}
2:
    cmp {buf}, {reg_end}
    bge 3f
    str {zero}, [{buf}]
    add {buf}, {buf}, 8
    b 2b
3:
    cmp {buf}, {end}
    bge 4f
    strb {zero:w}, [{buf}]
    add {buf}, {buf}, 1
    b 3b
4:
