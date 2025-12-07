#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, dx: isize, dy: isize) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}
