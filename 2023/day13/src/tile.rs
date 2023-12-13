#[derive(PartialEq, Eq)]
pub enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!(),
        }
    }
}
