#[derive(Copy, Clone)]
pub struct Area {
    pub pos1: (isize, isize),
    pub pos2: (isize, isize),
}

impl Area {
    pub fn new(pos1: (isize, isize), pos2: (isize, isize)) -> Self {
        Self { pos1, pos2 }
    }

    pub fn is_in_area(&self, x: isize, y: isize) -> bool {
        self.min_x() <= x && x <= self.max_x() && self.min_y() <= y && y <= self.max_y()
    }

    pub fn min_x(&self) -> isize {
        self.pos1.0.min(self.pos2.0)
    }

    pub fn max_x(&self) -> isize {
        self.pos1.0.max(self.pos2.0)
    }

    pub fn min_y(&self) -> isize {
        self.pos1.1.min(self.pos2.1)
    }

    pub fn max_y(&self) -> isize {
        self.pos1.1.max(self.pos2.1)
    }
}
