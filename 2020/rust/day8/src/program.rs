use crate::instruction::Instruction;

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub ip: i16,
    pub acc: i16,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ip: 0,
            acc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            instructions: self.instructions.to_vec(),
            ip: self.ip,
            acc: self.acc,
        }
    }
}