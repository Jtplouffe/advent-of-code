#[derive(PartialEq, Eq, Hash)]
pub enum Quadrant {
    OnAxis,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn quadrant(&self, grid_width: usize, grid_height: usize) -> Quadrant {
        let half_width = ((grid_width - 1) / 2) as isize;
        let half_height = ((grid_height - 1) / 2) as isize;

        if grid_width % 2 != 0 && self.x == half_width {
            assert_eq!(self.x, 50);
            return Quadrant::OnAxis;
        }

        if grid_height % 2 != 0 && self.y == half_height {
            assert_eq!(self.y, 51);
            return Quadrant::OnAxis;
        }

        let qx = self.x > half_width;
        let qy = self.y > half_height;

        match (qx, qy) {
            (false, false) => Quadrant::TopLeft,
            (true, false) => Quadrant::TopRight,
            (true, true) => Quadrant::BottomRight,
            (false, true) => Quadrant::BottomLeft,
        }
    }
}
