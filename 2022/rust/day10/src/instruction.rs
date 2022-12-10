#[derive(Debug)]
pub enum Instruction {
    ADDX(isize),
    NOOP,
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Self::ADDX(_) => 2,
            Self::NOOP => 1,
        }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let (instruction, arguments) = s.split_once(" ").unwrap_or((s, ""));

        match instruction {
            "addx" => Self::ADDX(arguments.parse().unwrap()),
            "noop" => Self::NOOP,
            _ => unreachable!(),
        }
    }
}
