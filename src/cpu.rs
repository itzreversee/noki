use core::panic;

pub struct CPU {
  registers: Vec<u16>,
  program_counter: u16,
  memory: Vec<u8>,
  running: bool
}

impl CPU {
  pub fn new() -> Self {
    Self {
      registers: vec![0; 12],
      program_counter: 0,
      memory: vec![0; 1024],
      running: false
    }
  }

  pub fn load_program(&mut self, program: &[u8]) {
    if program.len() > self.memory.capacity() {
      panic!("Trying to load program bigger than memory capacity!");
    }

    for (index, byte) in program.iter().enumerate() {
      self.memory[index] = *byte;
    }
  }

  pub fn step(&mut self) {
    // extract bytes and move pc
    let high = self.memory[usize::from(self.program_counter)];
    let low = self.memory[usize::from(self.program_counter + 1)];
    let instr = ((u16::from(high)) << 8) | (u16::from(low));
    self.program_counter += 2;
    
    let opcode = (instr >> 12) & 0xF;
    let dst = ((instr >> 9) & 0x7) as usize;
    let src = ((instr >> 6) & 0x7) as usize;
    let imm = (instr & 0x3F) as u8;
    
    println!("PC: {}; {}", self.program_counter - 2, opcode);

    match opcode {
      0x0 => (), // NOP
      0x1 => { // MOV
        self.registers[dst] = self.registers[src];
      },
      0x2 => { // LDI
        let value = sign_extend_6_bits(imm);
        self.registers[dst] = value as u16;
      },
      0x3 => { // ADD
        self.registers[dst] += self.registers[src];
      },
      0x4 => { // SUB
        self.registers[dst] -= self.registers[src];
      },
      0x5 => { // MUL
        self.registers[dst] *= self.registers[src];
      },
      0x6 => { // AND
        self.registers[dst] &= self.registers[src];
      },
      0x7 => { // OR
        self.registers[dst] |= self.registers[src];
      },
      0x8 => { // XOR
        self.registers[dst] ^= self.registers[src];
      },
      0x9 => { // CMP
        if self.registers[dst] == self.registers[src] {
          self.registers[11] = 0;
        }
      }
      0xA => { // JMP
        self.program_counter = self.registers[src];
      },
      0xB => { // JZ
        if self.registers[src] == 0 {
          self.program_counter = self.registers[dst];
        }
      },
      0xC => { // HLT
        self.running = false;
      },
      _ => {
        println!("Unknown opcode: {}", opcode);
        self.running = false;
      }
    };

    if usize::from(self.program_counter + 2) > self.memory.capacity() {
      self.running = false;
      return;
    }
  }

  // cpu features
  

  // helper functions
  pub fn setup(&mut self) {
    self.running = true;
  }
  
  pub fn begin(&mut self) {
    while self.running {
      self.step();
    }
  }

  pub fn dump_registers(&self) {
    for (i, reg) in self.registers.iter().enumerate() {
      println!("R{} : {}", i, reg)
    }
  }
}

fn sign_extend_6_bits(x: u8) -> i16 {
  if x & 0b0010_0000 != 0 {
    (x | 0b1100_0000) as i8 as i16
  } else {
    x as i8 as i16
  }
}