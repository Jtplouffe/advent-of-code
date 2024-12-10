use std::collections::{HashMap, HashSet};

use crate::position::Position;

pub struct Map {
    pub width: usize,
    pub height: usize,
    frequency_antennas: HashMap<char, Vec<Position>>,
}

impl Map {
    pub fn generate_antinodes(&self) -> HashMap<Position, char> {
        let mut antinodes = HashMap::new();

        for &frequency in self.frequency_antennas.keys() {
            let pairs = self.generate_antenna_pairs(frequency);

            for (left, right) in pairs {
                let (dx, dy) = left.distance(right);

                let left_antinode = left.add(dx, dy);
                let right_antinode = right.add(-dx, -dy);

                if left_antinode.is_within_rectangle(self.width, self.height) {
                    antinodes.insert(left_antinode, frequency);
                }

                if right_antinode.is_within_rectangle(self.width, self.height) {
                    antinodes.insert(right_antinode, frequency);
                }
            }
        }

        antinodes
    }

    pub fn generate_antinodes_with_harmonics(&self) -> HashMap<Position, char> {
        let mut antinodes = HashMap::new();

        for &frequency in self.frequency_antennas.keys() {
            let pairs = self.generate_antenna_pairs(frequency);

            for (left, right) in pairs {
                let (dx, dy) = left.distance(right);

                let mut left_antinode = left.clone();
                while left_antinode.is_within_rectangle(self.width, self.height) {
                    antinodes.insert(left_antinode, frequency);
                    left_antinode = left_antinode.add(dx, dy);
                }

                let mut right_antinode = right.clone();
                while right_antinode.is_within_rectangle(self.width, self.height) {
                    antinodes.insert(right_antinode, frequency);
                    right_antinode = right_antinode.add(-dx, -dy);
                }
            }
        }

        antinodes
    }

    fn generate_antenna_pairs(&self, frequency: char) -> HashSet<(&Position, &Position)> {
        let mut pairs = HashSet::new();
        let antennas = match self.frequency_antennas.get(&frequency) {
            Some(antennas) => antennas,
            None => return pairs,
        };

        for (i, left) in antennas.iter().enumerate() {
            for right in antennas[i + 1..].iter() {
                pairs.insert((left, right));
            }
        }

        pairs
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();

        let width = lines[0].len();
        let height = lines.len();

        let mut antennas = HashMap::<char, Vec<Position>>::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => {}
                    char => {
                        antennas
                            .entry(char)
                            .or_default()
                            .push(Position::new(x as isize, y as isize));
                    }
                };
            }
        }

        Self {
            width,
            height,
            frequency_antennas: antennas,
        }
    }
}
