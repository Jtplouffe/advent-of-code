#[derive(PartialEq, Eq)]
pub struct Cube {
    pub x1: isize,
    pub y1: isize,
    pub z1: isize,
    pub x2: isize,
    pub y2: isize,
    pub z2: isize,
}

impl Cube {
    pub fn copy_with_offset(&self, dx: isize, dy: isize, dz: isize) -> Self {
        Self {
            x1: self.x1 + dx,
            y1: self.y1 + dy,
            z1: self.z1 + dz,
            x2: self.x2 + dx,
            y2: self.y2 + dy,
            z2: self.z2 + dz,
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.x1 <= other.x2
            && self.x2 >= other.x1
            && self.y1 <= other.y2
            && self.y2 >= other.y1
            && self.z1 <= other.z2
            && self.z2 >= other.z1
    }

    pub fn is_supported_by(&self, other: &Self) -> bool {
        self.x1 <= other.x2
            && self.x2 >= other.x1
            && self.z1 <= other.z2
            && self.z2 >= other.z1
            && self.min_y() - 1 == other.max_y()
    }

    pub fn min_y(&self) -> isize {
        self.y1.min(self.y2)
    }

    pub fn max_y(&self) -> isize {
        self.y1.max(self.y2)
    }
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let parse_pos = |raw_pos: &str| {
            let mut split = raw_pos.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        };

        let (pos1, pos2) = value.split_once('~').unwrap();

        // For some reason y is not in the middle...
        let (x1, z1, y1) = parse_pos(pos1);
        let (x2, z2, y2) = parse_pos(pos2);

        Self {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        }
    }
}
