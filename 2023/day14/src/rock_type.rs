#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RockType {
    Round,
    Cube,
}

impl From<char> for RockType {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => unreachable!(),
        }
    }
}
