    str lr, [sp, #-8]!
    str fp, [sp, #-8]!
    str x10, [sp, #-8]!
    str x9, [sp, #-8]!

    // reserve memory for 3 locals
    // (i32 i32 i32 i32)
    // (x, y, sum, count)
    sub sp, sp, #32

    // frame
    mov fp, sp
    str xzr, [x29]
    str xzr, [x29, #8]
    str xzr, [x29, #16]
    str xzr, [x29, #24]

    // i32.const 10
    mov x9, #10
    str x9, [sp, #-8]!
    // local.set 0
    ldr x9, [sp], #8
    str x9, [fp, #0]

    // i32.const 20
    mov x9, #20
    str x9, [sp, #-8]!
    // local.set 1
    ldr x9, [sp], #8
    str x9, [fp, #8]

    // i32.const 0
    mov x9, #0
    str x9, [sp, #-8]!
    // local.set 2
    ldr x9, [sp], #8
    str x9, [fp, #16]

    // i32.const 0
    mov x9, #0
    str x9, [sp, #-8]!
    // local.set 3
    ldr x9, [sp], #8
    str x9, [fp, #24]

    /* beginning of block */

    // loop
    str lr, [sp, #-8]!
    bl #4

    // local.get 0
    ldr x9, [fp, #0]
    str x9, [sp, #-8]!
    // local.get 1
    ldr x9, [fp, #8]
    str x9, [sp, #-8]!
    // i32.add
    ldr x9, [sp], #8
    ldr x10, [sp], #8
    add x9, x9, x10
    str x9, [sp, #-8]!
    // local.get 2
    ldr x9, [fp, #8]
    str x9, [sp, #-8]!
    // i32.add
    ldr x9, [sp], #8
    ldr x10, [sp], #8
    add x9, x9, x10
    str x9, [sp, #-8]!
    // local.set 2
    ldr x9, [sp], #8
    str x9, [fp, #16]

    // local.get 3
    ldr x9, [fp, #24]
    str x9, [sp, #-8]!
    // i32.const 1
    mov x9, #1
    str x9, [sp, #-8]!
    // i32.sub
    ldr x10, [sp], #8
    ldr x9, [sp], #8
    sub x9, x9, x10
    str x9, [sp, #-8]!
    // local.set 3
    ldr x9, [sp], #8
    str x9, [fp, #16]
    // local.get 3
    ldr x9, [fp, #8]
    str x9, [sp, #-8]!
    // i32.const 0
    mov x9, #0
    str x9, [sp, #-8]!
    // i32.ne
    ldr x10, [sp], #8
    ldr x9, [sp], #8
    subs x9, x9, x10
    str x9, [sp, #-8]!
    // br_if
    /// assume that br* instruction appears only the end of a block
    ldr x9, [sp], #8
    subs x9, x9, xzr
    b.NE #4 //todo jump to end of bloc block

    // end
    br lr

    /// load 
    add sp, sp, #0
    ldr lr, [sp], #8
    /* end of block */

    
    /// end

    ldr x0, [sp], #8

    // clear frame
    add sp, sp, #16

    ldr x9, [sp], #8
    ldr x10, [sp], #8
    ldr fp, [sp], #8
    ldr lr, [sp], #8

    ret
