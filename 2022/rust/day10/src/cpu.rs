use crate::instruction::Instruction;
use std::collections::HashMap;

pub struct Cpu {
    pub x_register: isize,
    pub current_cycle: usize,
    x_register_history: HashMap<usize, isize>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            x_register: 1,
            current_cycle: 0,
            x_register_history: HashMap::new(),
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        self.x_register_history
            .insert(self.current_cycle, self.x_register);

        match instruction {
            Instruction::ADDX(value) => self.x_register += value,
            _ => {}
        }

        self.current_cycle += instruction.cycles();
    }

    pub fn signal_strength_sum_at_cycles(&self, cycles: &[usize]) -> isize {
        let mut sum = 0;

        for &cycle in cycles {
            sum += self.x_register_value_at_cycle(cycle) * cycle as isize;
        }

        sum
    }

    fn x_register_value_at_cycle(&self, cycle: usize) -> isize {
        let mut previous_entry_index = cycle - 1;
        while !self.x_register_history.contains_key(&previous_entry_index) {
            previous_entry_index -= 1;
        }

        *self.x_register_history.get(&previous_entry_index).unwrap()
    }

    pub fn generate_image(&self) -> String {
        let mut image = String::with_capacity(40 * 6);

        for y in 0..6 {
            for x in 0..40 {
                let cycle = y * 40 + x + 1;
                let x_register_value = self.x_register_value_at_cycle(cycle);

                if x_register_value - 1 <= x as isize && x_register_value + 1 >= x as isize {
                    image.push('█');
                } else {
                    image.push(' ');
                }
            }

            image.push('\n');
        }

        image
    }
}
