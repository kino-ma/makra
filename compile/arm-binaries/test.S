	str fp, [sp, #8]!
	str x2, [sp, #8]!
	str x1, [sp, #8]!

	mov x0, #10
	str x0, [sp, #8]!
	mov x0, #20
	str x0, [sp, #8]!
	ldr x1, [sp], #8
	ldr x2, [sp], #8
	add x0, x1, x2
	str x0, [sp, #8]!

	ldr x0, [sp], #8
	ldr x1, [sp], #8
	ldr x2, [sp], #8
	ldr fp, [sp], #8
