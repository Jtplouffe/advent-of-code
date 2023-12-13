#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringCondition {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}
