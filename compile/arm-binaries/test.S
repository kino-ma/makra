	PUSH {r2}
	PUSH {r1}

	mov r0, #10
	PUSH {r0}
	mov r0, #20
	PUSH {r0}
	POP {r1}
	POP {r2}
	add r0, r1, r2
	PUSH {r0}

	POP {r0}
	POP {r1}
	POP {r2}
