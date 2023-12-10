use crate::direction::Direction;

pub struct Pipe {
    from: Direction,
    to: Direction,
}

impl Pipe {
    pub fn connects_to(&self, direction: Direction) -> bool {
        self.from == direction || self.to == direction
    }

    pub fn other_direction(&self, direction: Direction) -> Direction {
        if self.from == direction {
            self.to
        } else {
            self.from
        }
    }
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self {
                from: Direction::North,
                to: Direction::South,
            },
            '-' => Self {
                from: Direction::East,
                to: Direction::West,
            },
            'L' => Self {
                from: Direction::North,
                to: Direction::East,
            },
            'J' => Self {
                from: Direction::North,
                to: Direction::West,
            },
            '7' => Self {
                from: Direction::South,
                to: Direction::West,
            },
            'F' => Self {
                from: Direction::South,
                to: Direction::East,
            },
            _ => unreachable!(),
        }
    }
}
