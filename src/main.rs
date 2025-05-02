mod cpu;

use core::panic;
use std::{
    env,
    fs
};

fn main() {
    let mut cpu = cpu::CPU::new();

    let args: Vec<String> = env::args().collect();
    let min_args = 2;

    if &args.len() < &min_args  {
        panic!("Usage: noki <program>");
    }

    println!("Loading program from: {}", &args[1]);

    let example_program: Vec<u8>  = fs::read(&args[1]).unwrap_or_else(|e| {
        panic!("Error while loading program! {e}!");
    });

    cpu.load_program(&example_program);

    cpu.setup();
    cpu.begin();

    cpu.dump_registers();
}