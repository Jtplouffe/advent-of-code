use std::collections::VecDeque;

use crate::position::Position;

pub struct Map {
    grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn trailhead_score_sum(&self) -> u32 {
        let mut score_sum = 0;

        for (y, line) in self.grid.iter().enumerate() {
            for (x, &height) in line.iter().enumerate() {
                if height != 0 {
                    continue;
                }

                let hiking_trails =
                    self.get_hiking_trails(Position::new(x as isize, y as isize), true);
                score_sum += hiking_trails.len() as u32;
            }
        }

        score_sum
    }

    pub fn trailhead_rating_sum(&self) -> u32 {
        let mut rating_sum = 0;

        for (y, line) in self.grid.iter().enumerate() {
            for (x, &height) in line.iter().enumerate() {
                if height != 0 {
                    continue;
                }

                let hiking_trails =
                    self.get_hiking_trails(Position::new(x as isize, y as isize), false);
                rating_sum += hiking_trails.len() as u32;
            }
        }

        rating_sum
    }

    fn get_hiking_trails(&self, trailhead: Position, single_path: bool) -> Vec<Vec<Position>> {
        let mut trails = Vec::<Vec<Position>>::new();

        let mut queue = VecDeque::new();
        queue.push_back(vec![trailhead]);

        while let Some(trail) = queue.pop_front() {
            let last_position = trail.last().unwrap();

            let current_height = match self.get_height(last_position) {
                Some(9) => {
                    let already_exists = single_path
                        && trails
                            .iter()
                            .any(|trail| trail.last().unwrap() == last_position);
                    if !already_exists {
                        trails.push(trail);
                    }

                    continue;
                }
                Some(height) => height,
                None => continue,
            };

            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let next_position = last_position.add(dx, dy);
                match self.get_height(&next_position) {
                    Some(next_height) if next_height == current_height + 1 => {
                        let mut trail = trail.clone();
                        trail.push(next_position);
                        queue.push_back(trail);
                    }
                    _ => {}
                }
            }
        }

        trails
    }

    fn get_height(&self, position: &Position) -> Option<u8> {
        if position.x < 0 || position.y < 0 {
            return None;
        }

        self.grid
            .get(position.y as usize)
            .and_then(|line| line.get(position.x as usize).copied())
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let grid = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Self { grid }
    }
}
