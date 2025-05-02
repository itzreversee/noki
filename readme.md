# noki
noki - minimal instruction set cpu emulator

## current instruction set:
NOP, MOV, LDI, ADD, SUB, MUL, AND, OR, XOR, JMP, CMP, JZ, HLT

## info
register count: 12
memory: 1024 (will be expanded to at least 64kb)

instruction format: [OPCODE 4 bits] [DST 3] [SRC 3] [EXT 6]
use `python3 compiler.py <in.s> <out.bin> to compile a program, see `programs/` directory for example programs.
to run the program, use `cargo run <bin>`

## program writing notes
when using CMP, the output will be set on R11
