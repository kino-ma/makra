    str lr, [sp, #-8]!
    str fp, [sp, #-8]!
    str x10, [sp, #-8]!
    str x9, [sp, #-8]!

    sub sp, sp, #16

    mov fp, sp
    str xzr, [x29]
    str xzr, [x29, #8]

