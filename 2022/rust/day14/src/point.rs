use std::ops::Sub;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn range_to_inclusive(&self, to: &Point) -> Vec<Self> {
        let point_count =
            self.x.max(to.x).sub(self.x.min(to.x)) + self.y.max(to.y).sub(self.y.min(to.y));
        let mut points = Vec::with_capacity(point_count);

        for y in self.y.min(to.y)..=self.y.max(to.y) {
            for x in self.x.min(to.x)..=self.x.max(to.x) {
                points.push(Self::new(x, y));
            }
        }

        points
    }

    pub fn add(&self, x: isize, y: isize) -> Self {
        Self {
            x: (self.x as isize + x) as usize,
            y: (self.y as isize + y) as usize,
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}
