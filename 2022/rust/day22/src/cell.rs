use crate::movement::Rotation;

pub enum Cell {
    Open,
    Wall,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Wall),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn with_move_in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::Est => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    Est,
    West,
}

impl Direction {
    pub fn with_rotation(&self, rotation: &Rotation) -> Self {
        let rotatory_index_change: isize = match rotation {
            Rotation::Left => -1,
            Rotation::Right => 1,
        };

        let new_rotatory_index =
            (self.rotatory_index() as isize + rotatory_index_change).rem_euclid(4) as usize;
        Self::from_rotatory_index(new_rotatory_index)
    }

    pub fn rotatory_index(&self) -> usize {
        match self {
            Self::North => 3,
            Self::Est => 0,
            Self::South => 1,
            Self::West => 2,
        }
    }

    pub fn from_rotatory_index(rotatory_index: usize) -> Self {
        match rotatory_index {
            3 => Self::North,
            0 => Self::Est,
            1 => Self::South,
            2 => Self::West,
            _ => unreachable!(),
        }
    }
}
