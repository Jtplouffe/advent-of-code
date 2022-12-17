#[derive(Debug)]
pub enum Movement {
    Left,
    Right,
}

impl From<char> for Movement {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}
