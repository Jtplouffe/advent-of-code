use crate::position::Position;

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Up,
    Direction::Right,
    Direction::Down,
];

pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn apply_offset(&self, (x, y): Position) -> Position {
        match self {
            Self::Left => (x - 1, y),
            Self::Up => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            _ => unreachable!(),
        }
    }
}
