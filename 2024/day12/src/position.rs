#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, dx: isize, dy: isize) -> Self {
        let x = (self.x as isize + dx) as usize;
        let y = (self.y as isize + dy) as usize;

        Self::new(x, y)
    }
}
