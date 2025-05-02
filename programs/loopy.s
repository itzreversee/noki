# R0 will be the jmp target
LDI R0, 6
# simple math to try to overflow the cpus memory
LDI R1, 2
LDI R2, 2
MUL R2, R1

# loop JMP
JMP R0

HLT