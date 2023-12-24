#[derive(PartialEq)]
pub struct HailStone {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl HailStone {
    pub fn intersection_2d(&self, other: &Self) -> Option<(f64, f64)> {
        let (m1, c1) = self.slope_and_intercept();
        let (m2, c2) = other.slope_and_intercept();

        if m1 == m2 {
            return None;
        }

        let x = (c2 - c1) / (m1 - m2);
        let y = m1 * x + c1;

        Some((x, y))
    }

    pub fn is_point_in_future(&self, (x, y): (f64, f64)) -> bool {
        !(self.dx > 0.0 && self.x > x
            || self.dx < 0.0 && self.x < x
            || self.dy > 0.0 && self.y > y
            || self.dy < 0.0 && self.y < y)
    }

    fn slope_and_intercept(&self) -> (f64, f64) {
        let delta = self.dy / self.dx;
        let c = self.y - delta * self.x;

        (delta, c)
    }
}

impl From<&str> for HailStone {
    fn from(value: &str) -> Self {
        let (position, delta) = value.split_once(" @ ").unwrap();

        let mut position_split = position.split(", ");
        let x = position_split.next().unwrap().trim().parse().unwrap();
        let y = position_split.next().unwrap().trim().parse().unwrap();
        let z = position_split.next().unwrap().trim().parse().unwrap();

        let mut delta_split = delta.split(", ");
        let dx = delta_split.next().unwrap().trim().parse().unwrap();
        let dy = delta_split.next().unwrap().trim().parse().unwrap();
        let dz = delta_split.next().unwrap().trim().parse().unwrap();

        Self {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        }
    }
}
