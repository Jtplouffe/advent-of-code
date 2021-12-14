use std::str::FromStr;

use crate::orientation::Orientation;

const FOLD_LINE_START: &str = "fold along ";

pub struct Fold {
    pub orientation: Orientation,
    pub position: usize,
}

impl Fold {
    pub fn is_in_fold(&self, dot: (usize, usize)) -> bool {
        match self.orientation {
            Orientation::Vertical => dot.1 > self.position,
            Orientation::Horizontal => dot.0 > self.position,
        }
    }

    pub fn dot_location_after_fold(&self, dot: (usize, usize)) -> Option<(usize, usize)> {
        if !self.is_in_fold(dot) {
            return Some(dot);
        }

        match self.orientation {
            Orientation::Vertical => {
                if dot.1 <= self.position * 2 {
                    Some((dot.0, self.position - (dot.1 - self.position)))
                } else {
                    None
                }
            }
            Orientation::Horizontal => {
                if dot.0 <= self.position * 2 {
                    Some((self.position - (dot.0 - self.position), dot.1))
                } else {
                    None
                }
            }
        }
    }
}

impl From<&str> for Fold {
    fn from(f: &str) -> Self {
        let f = f.replace(FOLD_LINE_START, "");
        let (orientation, position) = f.split_once("=").unwrap();
        let orientation = Orientation::from(orientation);
        let position = usize::from_str(position).unwrap();

        Self {
            orientation,
            position,
        }
    }
}
