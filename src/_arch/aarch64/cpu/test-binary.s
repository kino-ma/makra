.global _start

test_start:
        str w9, [sp, #-8]!
        str w0, [sp, #-8]!
        str w1, [sp, #-8]!
        //PUSH w9
        //PUSH w0
        //PUSH w1

        mov     w9, #0x1000
        movk    w9, #0x3f20
        mov w0, #0
        //movz x1, #:abs_g2:content
        //movk x1, #:abs_g1_nc:content
        //movk x1, #:abs_g0_nc:content
        movz x1, 'A'

//loop:
        //ldrb w11, [x1, x0]
        //strb w11, [x9]
        strb w1, [x9]
        ldr w1, [sp], #8
        ldr w0, [sp], #8
        ldr w9, [sp], #8

        //add w0, w0, #1
        //cmp w0, #20
        //bne loop

.data
        content: .ascii "hello from assembly\n"