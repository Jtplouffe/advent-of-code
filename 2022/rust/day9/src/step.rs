pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            _ => unreachable!(),
        }
    }
}

pub struct Step {
    pub direction: Direction,
    pub count: usize,
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        let (direction, count) = s.split_once(" ").unwrap();

        Self {
            direction: direction.into(),
            count: count.parse().unwrap(),
        }
    }
}
