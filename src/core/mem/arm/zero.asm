    eor {i}, {i}
    eor {byte}, {byte}
2:
    cmp {i}, {size}
    bge 3f
    strb {byte}, [{buf}, {i}]
    add {i}, 1
    bal 2b
3:
