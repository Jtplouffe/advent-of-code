#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

impl Direction {
    pub const fn values() -> [Self; 4] {
        [Self::Left, Self::Top, Self::Right, Self::Bottom]
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}
