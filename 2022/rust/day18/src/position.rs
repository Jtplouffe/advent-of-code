#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Position {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn neighbors(&self) -> Vec<Position> {
        vec![
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y, self.z + 1),
            Self::new(self.x, self.y, self.z - 1),
        ]
    }

    pub fn neighbors_with_direction(&self) -> Vec<(Position, Direction)> {
        vec![
            (Self::new(self.x + 1, self.y, self.z), Direction::Left),
            (Self::new(self.x - 1, self.y, self.z), Direction::Right),
            (Self::new(self.x, self.y + 1, self.z), Direction::Bottom),
            (Self::new(self.x, self.y - 1, self.z), Direction::Top),
            (Self::new(self.x, self.y, self.z + 1), Direction::Front),
            (Self::new(self.x, self.y, self.z - 1), Direction::Back),
        ]
    }

    pub fn is_in_bounds(&self, min: &Self, max: &Self) -> bool {
        min.x <= self.x
            && self.x <= max.x
            && min.y <= self.y
            && self.y <= max.y
            && min.z <= self.z
            && self.z <= max.z
    }
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let mut split = s.split(",");
        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}
