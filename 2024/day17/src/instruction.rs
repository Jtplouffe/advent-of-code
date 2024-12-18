#[derive(Clone)]
pub enum Instruction {
    ADV(u8),
    BDV(u8),
    CDV(u8),
    BXL(u8),
    BST(u8),
    JNZ(u8),
    BXC(),
    OUT(u8),
}

impl Instruction {
    pub fn new(opcode: u8, value: u8) -> Self {
        match opcode {
            0 => Self::ADV(value),
            1 => Self::BXL(value),
            2 => Self::BST(value),
            3 => Self::JNZ(value),
            4 => Self::BXC(),
            5 => Self::OUT(value),
            6 => Self::BDV(value),
            7 => Self::CDV(value),
            opcode => unreachable!("Unsupported opcode {opcode}"),
        }
    }
}
