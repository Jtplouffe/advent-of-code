use std::collections::HashMap;

use crate::rock_type::RockType;

const CYCLE_DETECTION_ROTATIONS: usize = 250;

type Position = (usize, usize);

#[derive(Clone)]
pub struct Platform {
    rocks: HashMap<Position, RockType>,
    width: usize,
    height: usize,
}

impl Platform {
    pub fn rotate(&mut self, rotations: usize) {
        if rotations == 0 {
            return;
        }

        for i in 1..=CYCLE_DETECTION_ROTATIONS {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            if i == rotations {
                return;
            }
        }

        let rocks_snapshot = self.rocks.clone();

        let mut current_rotations = CYCLE_DETECTION_ROTATIONS;
        loop {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            current_rotations += 1;

            if current_rotations == rotations {
                return;
            }

            if self.rocks == rocks_snapshot {
                break;
            }
        }

        let cycle_skip = current_rotations - CYCLE_DETECTION_ROTATIONS;
        for _ in 0..(rotations - CYCLE_DETECTION_ROTATIONS) % cycle_skip {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            current_rotations += 1;
        }
    }

    pub fn tilt_north(&mut self) {
        for y in 1..self.height {
            for x in 0..self.width {
                let position = (x, y);
                if Some(&RockType::Round) != self.rocks.get(&position) {
                    continue;
                }

                for dy in (-1..y as isize).rev() {
                    if dy >= 0 && !self.rocks.contains_key(&(x, dy as usize)) {
                        continue;
                    }

                    if dy < 0 || dy as usize != y {
                        let rock = self.rocks.remove(&position).unwrap();
                        self.rocks.insert((x, (dy + 1) as usize), rock);
                    }

                    break;
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        for y in (0..self.height - 1).rev() {
            for x in 0..self.width {
                let position = (x, y);
                if Some(&RockType::Round) != self.rocks.get(&position) {
                    continue;
                }

                for dy in y + 1..=self.height {
                    if dy < self.height && !self.rocks.contains_key(&(x, dy)) {
                        continue;
                    }

                    if dy == self.height || dy != y {
                        let rock = self.rocks.remove(&position).unwrap();
                        self.rocks.insert((x, dy - 1), rock);
                    }

                    break;
                }
            }
        }
    }

    pub fn tilt_east(&mut self) {
        for x in (0..(self.width - 1)).rev() {
            for y in 0..self.height {
                let position = (x, y);
                if Some(&RockType::Round) != self.rocks.get(&position) {
                    continue;
                }

                for dx in x + 1..=self.width {
                    if dx < self.width && !self.rocks.contains_key(&(dx, y)) {
                        continue;
                    }

                    if dx == self.width || dx != x {
                        let rock = self.rocks.remove(&position).unwrap();
                        self.rocks.insert((dx - 1, y), rock);
                    }

                    break;
                }
            }
        }
    }

    pub fn tilt_west(&mut self) {
        for x in 1..self.width {
            for y in 0..self.height {
                let position = (x, y);
                if Some(&RockType::Round) != self.rocks.get(&position) {
                    continue;
                }

                for dx in (-1..x as isize).rev() {
                    if dx >= 0 && !self.rocks.contains_key(&(dx as usize, y)) {
                        continue;
                    }

                    if dx == -1 || dx as usize != x {
                        let rock = self.rocks.remove(&position).unwrap();
                        self.rocks.insert(((dx + 1) as usize, y), rock);
                    }

                    break;
                }
            }
        }
    }

    pub fn north_support_beam_load(&self) -> usize {
        self.rocks
            .iter()
            .filter_map(|((_, y), rock_type)| match rock_type {
                RockType::Round => Some(self.height - y),
                RockType::Cube => None,
            })
            .sum()
    }
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        let rocks = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, char)| {
                    if char == '.' {
                        None
                    } else {
                        Some(((x, y), RockType::from(char)))
                    }
                })
            })
            .collect();

        Self {
            rocks,
            width,
            height,
        }
    }
}
