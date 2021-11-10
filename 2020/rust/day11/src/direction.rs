pub enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl Direction {
    pub fn value(&self) -> (isize, isize) {
        match self {
            Self::Top => (0, -1),
            Self::TopRight => (1, -1),
            Self::Right => (1, 0),
            Self::BottomRight => (1, 1),
            Self::Bottom => (0, 1),
            Self::BottomLeft => (-1, 1),
            Self::Left => (-1, 0),
            Self::TopLeft => (-1, -1),
        }
    }

    pub fn values() -> Vec<Self> {
        vec![
            Self::Top,
            Self::TopRight,
            Self::Right,
            Self::BottomRight,
            Self::Bottom,
            Self::BottomLeft,
            Self::Left,
            Self::TopLeft,
        ]
    }
}
