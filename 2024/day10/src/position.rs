#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, x: isize, y: isize) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}
