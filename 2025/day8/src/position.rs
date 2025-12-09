#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Position {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn euclidean_distance(&self, other: &Self) -> usize {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        (dx.pow(2) + dy.pow(2) + dz.pow(2)).abs().isqrt() as usize
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.x, self.y, self.z).cmp(&(other.x, other.y, other.z))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut split = value.split(',');
        let x = split
            .next()
            .expect("Invalid position")
            .parse()
            .expect("Invalid x");
        let y = split
            .next()
            .expect("Invalid position")
            .parse()
            .expect("Invalid y");
        let z = split
            .next()
            .expect("Invalid position")
            .parse()
            .expect("Invalid z");

        Self::new(x, y, z)
    }
}
