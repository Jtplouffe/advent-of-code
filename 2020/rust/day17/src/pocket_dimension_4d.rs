use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct PocketDimension4D {
    grid: HashSet<(isize, isize, isize, isize)> // x, y, z, w
}

impl PocketDimension4D {
    pub fn run(&mut self) {
        let mut new_grid = HashSet::new();

        for z in self.z_range() {
            for y in self.y_range() {
                for x in self.x_range() {
                    for w in self.w_range() {
                        let active_around = self.active_around(x, y, z, w);

                        if self.grid.contains(&(x, y, z, w)) {
                            if active_around == 2 || active_around == 3 {
                                new_grid.insert((x, y, z, w));
                            }
                        } else {
                            if active_around == 3 {
                                new_grid.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }

        self.grid = new_grid;
    }

    pub fn active_around(&self, x_pos: isize, y_pos: isize, z_pos: isize, w_pos: isize) -> usize {
        let mut count = 0;

        for z in z_pos - 1..=z_pos + 1 {
            for y in y_pos - 1..=y_pos + 1 {
                for x in x_pos - 1..=x_pos + 1 {
                    for w in w_pos - 1..=w_pos + 1 {
                        if !(x == x_pos && y == y_pos && z == z_pos && w == w_pos) && self.grid.contains(&(x, y, z, w)) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    pub fn x_range(&self) -> RangeInclusive<isize> {
        let min = self.grid.iter().map(|(x, _, _, _)| x).min().unwrap();
        let max = self.grid.iter().map(|(x, _, _, _)| x).max().unwrap();
        min - 1..=max + 1 // +/- 1 on each side for infinity
    }

    pub fn y_range(&self) -> RangeInclusive<isize> {
        let min = self.grid.iter().map(|(_, y, _, _)| y).min().unwrap();
        let max = self.grid.iter().map(|(_, y, _, _)| y).max().unwrap();
        min - 1..=max + 1 // +/- 1 on each side for infinity
    }

    pub fn z_range(&self) -> RangeInclusive<isize> {
        let min = self.grid.iter().map(|(_, _, z, _)| z).min().unwrap();
        let max = self.grid.iter().map(|(_, _, z, _)| z).max().unwrap();
        min - 1..=max + 1 // +/- 1 on each side for infinity
    }

    pub fn w_range(&self) -> RangeInclusive<isize> {
        let min = self.grid.iter().map(|(_, _, _, w)| w).min().unwrap();
        let max = self.grid.iter().map(|(_, _, _, w)| w).max().unwrap();
        min - 1..=max + 1 // +/- 1 on each side for infinity
    }

    pub fn active_count(&self) -> usize {
        self.grid.len()
    }
}

impl From<&str> for PocketDimension4D {
    fn from(s: &str) -> Self {
        let mut grid = HashSet::new();

        for (line_index, line) in s.lines().enumerate() {
            for (char_index, char) in line.chars().enumerate() {
                if char == '#' {
                    grid.insert((char_index as isize, line_index as isize, 0, 0));
                }
            }
        }

        Self { grid }
    }
}