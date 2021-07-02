	PUSH {r0}
	PUSH {r1}
	PUSH {r2}

	PUSH 10
	PUSH 20
	POP {r1}
	POP {r2}
	add r0, r1, r2
