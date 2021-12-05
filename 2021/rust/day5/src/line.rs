use crate::point::Point;

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn is_linear(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    pub fn get_points(&self) -> Vec<Point> {
        let mut points = vec![];

        let min_x = self.start.x.min(self.end.x);
        let max_x = self.start.x.max(self.end.x);
        let min_y = self.start.y.min(self.end.y);
        let max_y = self.start.y.max(self.end.y);

        if self.start.x != self.end.x && self.start.y != self.end.y {
            let x_delta  = if self.start.x < self.end.x {
                1
            } else {
                -1
            };

            let y_delta = if self.start.y < self.end.y {
                1
            } else {
                -1
            };

            for i in 0..=(max_x - min_x) {
                points.push(Point::new(self.start.x + i * x_delta, self.start.y + i * y_delta));
            }
        } else if self.start.x != self.end.x {
            for x in min_x..=max_x {
                points.push(Point::new(x, self.start.y));
            }
        } else if self.start.y != self.end.y {
            for y in min_y..=max_y {
                points.push(Point::new(self.start.x, y));
            }
        }

        points
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let (start, end) = s.split_once(" -> ").unwrap();

        Self {
            start: Point::from(start),
            end: Point::from(end),
        }
    }
}
