
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, x: isize, y: isize) -> Self {
        Self::new(
            (self.x as isize + x) as usize,
            (self.y as isize + y) as usize,
        )
    }
}
