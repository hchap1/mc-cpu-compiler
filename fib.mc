0 -> A0
0 -> A1
ADD -> R0
1 -> A1
ADD -> R1
loop:
	R1 -> A0
	R0 -> A1
	ADD -> R1
	0 -> A1
	ADD -> R0
	R1 -> I0
