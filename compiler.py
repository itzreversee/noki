import sys

OPCODES = {
    "NOP": 0x0,
    "MOV": 0x1,
    "LDI": 0x2,
    "ADD": 0x3,
    "SUB": 0x4,
    "MUL": 0x5,
    "AND": 0x6,
    "OR": 0x7,
    "XOR": 0x8,
    "CMP": 0x9,
    "JMP": 0xA,
    "JZ": 0xB,
    "HLT": 0xC
}

REGISTERS = {
    f"R{i}": i for i in range(12)
}

def assemble_line(line):
    parts = line.strip().replace(",", "").split()
    if not parts:
        return None

    op = parts[0].upper()

    if op == "LDI":
        dst = REGISTERS[parts[1]]
        imm = int(parts[2])
        if imm < -32 or imm > 31:
            raise ValueError("imm out of range (-32 to 31)")
        imm &= 0x3F
        return (OPCODES[op] << 12) | (dst << 9) | imm
    
    elif op in {"ADD", "SUB", "MUL", "AND", "MOV", "OR", "XOR", "CMP", "JZ"}:
        dst = REGISTERS[parts[1]]
        src = REGISTERS[parts[2]]
        return (OPCODES[op] << 12) | (dst << 9) | (src << 6)

    elif op == "JMP":
        dst = REGISTERS[parts[1]]
        return (OPCODES[op] << 12) | (dst << 9)

    elif op == "HLT" or op == "NOP":
        return OPCODES[op] << 12

    else:
        raise ValueError(f"Unknown instruction: {op}")

def assemble_file(path):
    with open(path) as f:
        lines = f.readlines()

    program = []
    for line in lines:
        if line.strip() == "" or line.strip().startswith("#"):
            continue
        instr = assemble_line(line)
        if instr is not None:
            program.append(instr)
    return program

def save_binary(program, path):
    with open(path, "wb") as f:
        for instr in program:
            f.write(instr.to_bytes(2, "big"))

if __name__ == '__main__':
    file_input = sys.argv[1];
    file_output = sys.argv[2];
    program = assemble_file(file_input)
    save_binary(program, file_output)