use std::collections::{HashSet, VecDeque};

use crate::{dig_plan_line::DigPlanLine, direction::DIRECTIONS, position::Position};

pub struct Lagoon {
    lines: Vec<DigPlanLine>,
    digged_tiles: HashSet<Position>,
}

impl Lagoon {
    pub fn dig(&mut self) {
        let mut current_position = (0, 0);
        self.digged_tiles.insert(current_position);

        for line in &self.lines {
            for _ in 1..=line.length {
                current_position = line.direction.apply_offset(current_position);

                self.digged_tiles.insert(current_position);
            }
        }

        self.dig_out_inside();
    }

    pub fn capacity(&self) -> usize {
        self.digged_tiles.len()
    }

    fn dig_out_inside(&mut self) {
        let mut queue = VecDeque::new();

        let (min_x, min_y) = self.min_bounds();
        let (max_x, max_y) = self.max_bounds();
        for y in [min_y, max_y] {
            for x in min_x..=max_x {
                let position = (x, y);
                if !self.digged_tiles.contains(&position) {
                    queue.push_back(position);
                }
            }
        }

        for x in [min_x, max_x] {
            for y in min_y + 1..max_y {
                let position = (x, y);
                if !self.digged_tiles.contains(&position) {
                    queue.push_back(position);
                }
            }
        }

        let mut accessible_tiles_from_outside = HashSet::new();
        while let Some(position) = queue.pop_front() {
            if self.digged_tiles.contains(&position) {
                continue;
            }

            if accessible_tiles_from_outside.contains(&position) {
                continue;
            }

            accessible_tiles_from_outside.insert(position);

            for direction in DIRECTIONS {
                let next_position = direction.apply_offset(position);

                if next_position.0 < min_x
                    || next_position.0 > max_x
                    || next_position.1 < min_y
                    || next_position.1 > max_y
                {
                    continue;
                }

                if self.digged_tiles.contains(&next_position)
                    || accessible_tiles_from_outside.contains(&next_position)
                {
                    continue;
                }

                queue.push_back(next_position);
            }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let position = (x, y);
                if !accessible_tiles_from_outside.contains(&position) {
                    self.digged_tiles.insert(position);
                }
            }
        }
    }

    fn min_bounds(&self) -> Position {
        self.digged_tiles
            .iter()
            .fold((0, 0), |(min_x, min_y), &(x, y)| {
                if x < min_x && min_y < y {
                    (x, y)
                } else if x < min_x {
                    (x, min_y)
                } else if y < min_y {
                    (min_x, y)
                } else {
                    (min_x, min_y)
                }
            })
    }

    fn max_bounds(&self) -> Position {
        self.digged_tiles
            .iter()
            .fold((0, 0), |(max_x, max_y), &(x, y)| {
                if x > max_x && max_y > y {
                    (x, y)
                } else if x > max_x {
                    (x, max_y)
                } else if y > max_y {
                    (max_x, y)
                } else {
                    (max_x, max_y)
                }
            })
    }
}

impl From<&str> for Lagoon {
    fn from(value: &str) -> Self {
        let lines = value.lines().map(DigPlanLine::from).collect();

        Self {
            lines,
            digged_tiles: HashSet::new(),
        }
    }
}
