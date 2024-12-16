#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Wall,
    SmallBox,
    BigBoxFirstHalf,
    BigBoxSecondHalf,
}

impl Tile {
    pub fn from_char(char: char) -> Option<Self> {
        match char {
            '#' => Some(Self::Wall),
            'O' => Some(Self::SmallBox),
            '.' => None,
            _ => unreachable!(),
        }
    }
}
