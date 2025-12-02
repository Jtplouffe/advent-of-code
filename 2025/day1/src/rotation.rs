pub enum Rotation {
    Left(usize),
    Right(usize),
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let (direction, count) = value.split_at(1);
        let count = count.parse::<_>().unwrap();

        match direction {
            "L" => Self::Left(count),
            "R" => Self::Right(count),
            _ => unreachable!(),
        }
    }
}
