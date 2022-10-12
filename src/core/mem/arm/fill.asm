    eor {i}, {i}
2:
    cmp {i}, {size}
    bge 3f
    strb {fill}, [{buf}, {i}]
    add {i}, 1
    bal 2b
3:
