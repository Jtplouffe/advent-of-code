#[derive(PartialEq, Clone)]
pub enum Place {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Place {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}