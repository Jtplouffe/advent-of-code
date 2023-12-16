use crate::position::Position;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

impl Direction {
    pub fn next_position(&self, (x, y): Position) -> Position {
        match self {
            Self::Left => (x - 1, y),
            Self::Top => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Bottom => (x, y + 1),
        }
    }

    pub fn is_vertical(&self) -> bool {
        self == &Self::Top || self == &Self::Bottom
    }
}
