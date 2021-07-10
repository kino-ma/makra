    str lr, [sp, #-8]!
    str fp, [sp, #-8]!
    str x11, [sp, #-8]!
    str x10, [sp, #-8]!
    str x9, [sp, #-8]!

    // frame
    mov fp, sp

    // reserve memory for 2 locals
    sub sp, sp, #16

    // i32.const 10
    mov x9, #10
    str x9, [sp, #-8]!
    // local.set 0
    ldr x9, [sp], #8
    str x9, [fp, #0]

    // i32.const 20
    mov x9, #20
    str x9, [sp, #-8]!
    // local.set 0
    ldr x9, [sp], #8
    str x9, [fp, #0]

    // i32.add
    ldr x11, [sp], #8
    ldr x10, [sp], #8
    add x9, x9, x10
    str x0, [sp, #-8]!

    ldr x0, [sp], #8
    ldr x9, [sp], #8
    ldr x10, [sp], #8
    ldr x11, [sp], #8
    ldr fp, [sp], #8
    ldr lr, [sp], #8

    ret
