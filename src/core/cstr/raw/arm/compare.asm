    mov {is_eq}, 1
    cmp {cstr1}, {cstr2}
    beq 4f
2:
    ldrb {ch1}, [{cstr1}]
    ldrb {ch2}, [{cstr2}]
    cmp {ch1}, {ch2}
    bne 3f
    tst {ch1}, {ch1}
    beq 4f
    add {cstr1}, {cstr1}, 1
    add {cstr2}, {cstr2}, 1
    b 2b
3:
    eor {is_eq}, {is_eq}, {is_eq}
4:
