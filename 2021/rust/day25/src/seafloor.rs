use std::collections::{HashMap, HashSet};

enum SeaCucumberOrientation {
    East,
    South,
}

impl SeaCucumberOrientation {
    pub fn is_east(&self) -> bool {
        matches!(self, SeaCucumberOrientation::East)
    }

    pub fn is_south(&self) -> bool {
        matches!(self, SeaCucumberOrientation::South)
    }
}

pub struct Seafloor {
    sea_cucumbers: HashMap<(usize, usize), SeaCucumberOrientation>,
    width: usize,
    height: usize,
}

impl Seafloor {
    // returns true if no sea cucumber has moved
    pub fn step(&mut self) -> bool {
        let mut sea_cucumbers_to_move_east = Vec::<(usize, usize)>::new();
        for position in self.get_east_sea_cucumbers() {
            if !self.sea_cucumbers.contains_key(&((position.0 + 1) % self.width, position.1)) {
                sea_cucumbers_to_move_east.push(position);
            }
        }

        for sea_cucumber_to_move_east in &sea_cucumbers_to_move_east {
            let new_position = ((sea_cucumber_to_move_east.0 + 1) % self.width, sea_cucumber_to_move_east.1);

            self.sea_cucumbers.remove(&sea_cucumber_to_move_east);
            self.sea_cucumbers.insert(new_position, SeaCucumberOrientation::East);
        }

        let mut sea_cucumbers_to_move_south = Vec::<(usize, usize)>::new();
        for position in self.get_south_sea_cucumbers() {
            if !self.sea_cucumbers.contains_key(&(position.0, (position.1 + 1) % self.height)) {
                sea_cucumbers_to_move_south.push(position);
            }
        }

        for sea_cucumber_to_move_south in &sea_cucumbers_to_move_south {
            let new_position = (sea_cucumber_to_move_south.0, (sea_cucumber_to_move_south.1 + 1) % self.height);

            self.sea_cucumbers.remove(&sea_cucumber_to_move_south);
            self.sea_cucumbers.insert(new_position, SeaCucumberOrientation::South);
        }

        !sea_cucumbers_to_move_east.is_empty() || !sea_cucumbers_to_move_south.is_empty()
    }

    fn get_east_sea_cucumbers(&self) -> HashSet<(usize, usize)> {
        self.sea_cucumbers.iter().filter(|(_, orientation)| orientation.is_east()).map(|(position, _)| *position).collect()
    }

    fn get_south_sea_cucumbers(&self) -> HashSet<(usize, usize)> {
        self.sea_cucumbers.iter().filter(|(_, orientation)| orientation.is_south()).map(|(position, _)| *position).collect()
    }
}

impl From<&str> for Seafloor {
    fn from(s: &str) -> Self {
        let mut sea_cucumbers = HashMap::new();

        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'v' {
                    sea_cucumbers.insert((x, y), SeaCucumberOrientation::South);
                } else if c == '>' {
                    sea_cucumbers.insert((x, y), SeaCucumberOrientation::East);
                }
            }
        }

        Self { sea_cucumbers, width, height }
    }
}
