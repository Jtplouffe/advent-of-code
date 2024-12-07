#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn mov(&self, direction: &Direction) -> Self {
        let delta = direction.delta();
        self.add(delta)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn delta(&self) -> Position {
        match self {
            Direction::North => Position::new(0, -1),
            Direction::South => Position::new(0, 1),
            Direction::East => Position::new(1, 0),
            Direction::West => Position::new(-1, 0),
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::South => Self::West,
            Direction::East => Self::South,
            Direction::West => Self::North,
        }
    }
}
