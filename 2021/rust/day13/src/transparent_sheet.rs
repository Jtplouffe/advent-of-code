use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::fold::Fold;

pub struct TransparentSheet {
    dots: HashSet<(usize, usize)>,
    folds: Vec<Fold>,
}

impl TransparentSheet {
    pub fn fold_once(&mut self) {
        debug_assert!(!self.folds.is_empty());
        let fold = self.folds.remove(0);
        self.dots = self
            .dots
            .iter()
            .filter_map(|&dot| fold.dot_location_after_fold(dot))
            .collect();
    }

    pub fn fold_rest(&mut self) {
        while !self.folds.is_empty() {
            self.fold_once();
        }
    }

    pub fn dot_count(&self) -> usize {
        self.dots.len()
    }
}

impl From<&str> for TransparentSheet {
    fn from(s: &str) -> Self {
        let (dots, folds) = s.split_once("\n\n").unwrap();

        let dots = dots
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                (usize::from_str(x).unwrap(), usize::from_str(y).unwrap())
            })
            .collect();
        let folds = folds.lines().map(Fold::from).collect();

        Self { folds, dots }
    }
}

impl Display for TransparentSheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|&d| d.0).max().unwrap();
        let max_y = self.dots.iter().map(|&d| d.1).max().unwrap();

        let mut output = String::new();

        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.contains(&(x, y)) {
                    output.push('▓');
                } else {
                    output.push(' ');
                }
            }

            if y < max_y {
                output.push('\n');
            }
        }

        write!(f, "{}", output)
    }
}