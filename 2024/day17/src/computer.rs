use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::instruction::Instruction;

#[derive(Clone)]
pub struct Computer {
    raw_program: String,

    register_a: usize,
    register_b: usize,
    register_c: usize,
    ip: usize,

    program: Vec<Instruction>,
}

impl Computer {
    pub fn execute(&mut self) -> Vec<String> {
        let mut outputs = Vec::new();

        while self.ip < self.program.len() {
            if let Some(out) = self.execute_next_instruction() {
                outputs.push(out.to_string());
            }
        }

        outputs
    }

    pub fn find_register_a_value_for_prgoram_replication(&self) -> usize {
        let register_a = (0..).par_bridge().find_map_any(|register_a| {
            if register_a % 10_000_000 == 0 {
                println!("{register_a}");
            }
            let mut instance = self.clone();
            instance.register_a = register_a;

            let output = instance.execute();
            if output.join(",") == self.raw_program {
                Some(register_a)
            } else {
                None
            }
        });

        if let Some(register_a) = register_a {
            return register_a;
        }

        unreachable!()
    }

    fn execute_next_instruction(&mut self) -> Option<usize> {
        let mut increment_ip = true;
        let mut output = None;

        let instruction = &self.program[self.ip];
        match instruction {
            &Instruction::ADV(combo_operand)
            | &Instruction::BDV(combo_operand)
            | &Instruction::CDV(combo_operand) => {
                let combo_operand_value = self.get_combo_operand_value(combo_operand);

                let numerator = self.register_a;
                let denominator = 2usize.pow(combo_operand_value as u32);
                let result = numerator / denominator;

                match instruction {
                    Instruction::ADV(_) => self.register_a = result,
                    Instruction::BDV(_) => self.register_b = result,
                    Instruction::CDV(_) => self.register_c = result,
                    _ => unreachable!(),
                }
            }
            &Instruction::BXL(literal) => {
                self.register_b ^= literal as usize;
            }
            &Instruction::BST(combo_operand) => {
                let combo_operand_value = self.get_combo_operand_value(combo_operand);
                self.register_b = combo_operand_value % 8;
            }
            &Instruction::JNZ(literal) => {
                if self.register_a != 0 {
                    assert!(literal % 2 == 0);
                    self.ip = literal as usize / 2;
                    increment_ip = false;
                }
            }
            &Instruction::BXC() => {
                self.register_b ^= self.register_c;
            }
            &Instruction::OUT(combo_operand) => {
                let combo_operand_value = self.get_combo_operand_value(combo_operand);
                output = Some(combo_operand_value % 8);
            }
        }

        if increment_ip {
            self.ip += 1;
        }

        output
    }

    fn get_combo_operand_value(&self, combo_operand: u8) -> usize {
        match combo_operand {
            0..=3 => combo_operand as usize,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            combo_operand => unreachable!("Unsupported combo operand '{combo_operand}"),
        }
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let (registers, program) = value.split_once("\n\n").unwrap();

        let mut register_a = None;
        let mut register_b = None;
        let mut register_c = None;
        for register in registers.lines() {
            let (name, value) = register.split_once(": ").expect("Malformed register");
            let value = value.parse::<usize>().expect("Malformed register value");

            match name.to_lowercase().as_str() {
                "register a" => register_a = Some(value),
                "register b" => register_b = Some(value),
                "register c" => register_c = Some(value),
                name => unreachable!("Invalid register name '{name}'"),
            }
        }

        let (_, raw_program) = program.split_once(": ").expect("Malformed program");
        let parsed_raw_program: Vec<_> = raw_program
            .split(',')
            .map(|value| value.parse::<u8>().expect("Invalid value"))
            .collect();

        let mut program = Vec::new();
        for i in (0..parsed_raw_program.len()).step_by(2) {
            let opcode = parsed_raw_program[i];
            let value = parsed_raw_program[i + 1];

            program.push(Instruction::new(opcode, value));
        }

        Self {
            raw_program: raw_program.to_string(),
            register_a: register_a.expect("Failed to initialize register a"),
            register_b: register_b.expect("Failed to initialize register b"),
            register_c: register_c.expect("Failed to initialize register c"),
            ip: 0,
            program,
        }
    }
}
