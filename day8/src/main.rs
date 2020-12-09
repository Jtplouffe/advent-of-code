#![feature(str_split_once)]

use crate::instruction::Instruction;
use crate::processor::Processor;
use crate::program::Program;

mod instruction;
mod processor;
mod program;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    let mut program = Program::new(instructions);

    println!("Part 1: {}", Processor::execute(&mut program).0);
    program.reset();
    println!("Part 2: {}", Processor::execute_with_error_correction(&mut program))
}
