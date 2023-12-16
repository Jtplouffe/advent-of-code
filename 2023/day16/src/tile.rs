use crate::{direction::Direction, position::Position};

#[derive(Debug)]
pub enum Tile {
    Mirror(Mirror),
    Splitter(Splitter),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '/' => Self::Mirror(Mirror::Forward),
            '\\' => Self::Mirror(Mirror::Backward),
            '|' => Self::Splitter(Splitter::Vertical),
            '-' => Self::Splitter(Splitter::Horizontal),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Mirror {
    Forward,
    Backward,
}

impl Mirror {
    pub fn next_directed_position(
        &self,
        (x, y): Position,
        direction: Direction,
    ) -> (Position, Direction) {
        match self {
            Self::Forward => match direction {
                Direction::Left => ((x, y + 1), Direction::Bottom),
                Direction::Bottom => ((x - 1, y), Direction::Left),
                Direction::Right => ((x, y - 1), Direction::Top),
                Direction::Top => ((x + 1, y), Direction::Right),
            },
            Self::Backward => match direction {
                Direction::Left => ((x, y - 1), Direction::Top),
                Direction::Top => ((x - 1, y), Direction::Left),
                Direction::Right => ((x, y + 1), Direction::Bottom),
                Direction::Bottom => ((x + 1, y), Direction::Right),
            },
        }
    }
}

#[derive(Debug)]
pub enum Splitter {
    Vertical,
    Horizontal,
}

impl Splitter {
    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical)
    }

    pub fn next_positions(
        &self,
        position: Position,
    ) -> ((Position, Direction), (Position, Direction)) {
        match self {
            Self::Vertical => (
                (Direction::Top.next_position(position), Direction::Top),
                (Direction::Bottom.next_position(position), Direction::Bottom),
            ),
            Self::Horizontal => (
                (Direction::Left.next_position(position), Direction::Left),
                (Direction::Right.next_position(position), Direction::Right),
            ),
        }
    }
}
