type N = i32;

pub enum Action {
    North(N),
    South(N),
    East(N),
    West(N),
    Left(N),
    Right(N),
    Forward(N),
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let action_char = s.chars().nth(0).unwrap();
        let value = s[1..].parse().unwrap();

        match action_char {
            'N' => Self::North(value),
            'S' => Self::South(value),
            'E' => Self::East(value),
            'W' => Self::West(value),
            'L' => Self::Left(value),
            'R' => Self::Right(value),
            'F' => Self::Forward(value),
            _ => unreachable!(),
        }
    }
}
