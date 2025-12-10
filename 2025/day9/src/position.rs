#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, dx: isize, dy: isize) -> Self {
        Self::new(
            (self.x as isize + dx) as usize,
            (self.y as isize + dy) as usize,
        )
    }

    pub fn euclidean_distance(&self, other: &Self) -> usize {
        let dx = other.x as isize - self.x as isize;
        let dy = other.y as isize - self.y as isize;

        (dx.pow(2) + dy.pow(2)).abs().isqrt() as usize
    }

    pub fn area(&self, other: &Self) -> usize {
        let width = (self.x as isize - other.x as isize).unsigned_abs() + 1;
        let height = (self.y as isize - other.y as isize).unsigned_abs() + 1;

        width * height
    }

    pub fn is_in_area(&self, other: &Self, &Position { x, y }: &Position) -> bool {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);

        min_x <= x && x <= max_x && min_y <= y && y <= max_y
    }

    pub fn unit_direction_delta(&self, other: &Self) -> (isize, isize) {
        let dx = if self.x < other.x {
            1
        } else if self.x > other.x {
            -1
        } else {
            0
        };

        let dy = if self.y < other.y {
            1
        } else if self.y > other.y {
            -1
        } else {
            0
        };

        (dx, dy)
    }

    pub fn iter_area_position(&self, other: &Self) -> impl Iterator<Item = Self> {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);

        (min_y..=max_y).flat_map(move |y| (min_x..=max_x).map(move |x| Self::new(x, y)))
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').expect("Invaild position");
        let x = x.parse().expect("Invalid x");
        let y = y.parse().expect("Invalid y");

        Self::new(x, y)
    }
}
