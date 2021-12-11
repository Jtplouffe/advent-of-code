use crate::instruction::Instruction;

pub struct ProgramLine {
    pub instructions: Vec<Instruction>,
}

impl ProgramLine {
    pub fn first_corrupted_instruction(&self) -> Option<&Instruction> {
        let mut instruction_stack = Vec::<&Instruction>::new();

        for instruction in &self.instructions {
            if instruction.is_opening() {
                instruction_stack.push(instruction)
            } else {
                let matching_instruction = instruction_stack.pop().unwrap();
                if &instruction.opposite() != matching_instruction {
                    return Some(instruction);
                }
            }
        }

        None
    }

    pub fn autocomplete_score(&self) -> Option<u64> {
        let mut instruction_stack = Vec::<&Instruction>::new();

        for instruction in &self.instructions {
            if instruction.is_opening() {
                instruction_stack.push(instruction);
            } else {
                let matching_instruction = instruction_stack.pop().unwrap();
                if &instruction.opposite() != matching_instruction {
                    return None;
                }
            }
        }

        instruction_stack
            .iter()
            .rev()
            .map(|i| i.opposite().autocomplete_score() as u64)
            .reduce(|score, s| score * 5 + s)
    }
}

impl From<&str> for ProgramLine {
    fn from(s: &str) -> Self {
        let instructions = s.chars().map(Instruction::from).collect();

        Self { instructions }
    }
}
