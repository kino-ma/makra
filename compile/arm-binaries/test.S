	PUSH {r0}
	PUSH {r1}
	PUSH {r2}

	mov r0, #10
	PUSH {r0}
	mov r0, #10
	PUSH {r0}
	POP {r1}
	POP {r2}
	add r0, r1, r2
