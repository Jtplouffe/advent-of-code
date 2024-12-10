use std::ops::Sub;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> (isize, isize) {
        let dx = self.x.sub(other.x);
        let dy = self.y.sub(other.y);

        (dx, dy)
    }

    pub fn add(&self, x: isize, y: isize) -> Self {
        Self::new(self.x + x, self.y + y)
    }

    pub fn is_within_rectangle(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < width as isize && self.y < height as isize
    }
}
