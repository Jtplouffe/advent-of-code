#[derive(Clone)]
pub enum Instruction {
    ACC(i16),
    JMP(i16),
    NOP(i16),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let (raw_type, raw_value) = s.split_once(" ").unwrap();
        let value: i16 = raw_value.parse().unwrap();

        match raw_type {
            "acc" => Self::ACC(value),
            "jmp" => Self::JMP(value),
            "nop" => Self::NOP(value),
            _ => unreachable!(),
        }
    }
}
