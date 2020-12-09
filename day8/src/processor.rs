use crate::instruction::{Instruction};
use crate::program::Program;

pub struct Processor;

impl Processor {
    pub fn execute(program: &mut Program) -> (i16, bool) {
        let mut processed_ip = Vec::<i16>::new();

        while !processed_ip.contains(&program.ip) && program.instructions.get(program.ip as usize).is_some() {
            let instruction = program.instructions.get(program.ip as usize).unwrap();
            processed_ip.push(program.ip);

            match instruction {
                Instruction::ACC(n) => {
                    program.acc += n;
                    program.ip += 1;
                }
                Instruction::JMP(n) => program.ip += n,
                Instruction::NOP(_) => program.ip += 1,
            }
        }

        (program.acc, processed_ip.contains(&program.ip))
    }

    pub fn execute_with_error_correction(program: &Program) -> i16 {
        for i in (0..program.instructions.len()).rev() {
            let new_instruction = match program.instructions[i] {
                Instruction::JMP(value) => Instruction::NOP(value),
                Instruction::NOP(value) => Instruction::JMP(value),
                _ => continue,
            };

            let mut temp_program = program.clone();
            temp_program.instructions[i] = new_instruction;

            let execution_result = Self::execute(&mut temp_program);
            if !execution_result.1 {
                return execution_result.0;
            }
        }

        panic!("Could not find a way to fix the program")
    }
}

