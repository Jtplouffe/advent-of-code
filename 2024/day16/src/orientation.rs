#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn values() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    pub fn delta(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Self::West | Self::East => true,
            Self::North | Self::South => false,
        }
    }

    pub fn quarter_rotations(&self, other: &Self) -> usize {
        if self == other {
            return 0;
        }

        if self.is_horizontal() == other.is_horizontal() {
            2
        } else {
            1
        }
    }
}
