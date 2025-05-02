# Mult values
LDI R0, 2
LDI R1, 2

# Target value
LDI R2, 16

# instruction posiitons, MUL and HLT
LDI R6, 10
LDI R7, 18

# multiply R1 <- R0 and check if it is equal to 16
MUL R1, R0
CMP R2, R1
JZ R7, R11 
JMP R6

# stop
HLT