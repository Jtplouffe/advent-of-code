use std::collections::{HashMap, HashSet, VecDeque};

use crate::{orientation::Orientation, position::Position};

pub struct Maze {
    walls: HashSet<Position>,
    starting_position: Position,
    ending_position: Position,
}

impl Maze {
    pub fn find_lowest_score(&mut self) -> (usize, usize) {
        let mut lowest_scores = HashMap::<(Position, Orientation), usize>::new();
        let mut final_paths = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back((
            self.starting_position,
            Orientation::East,
            0,
            vec![self.starting_position],
        ));

        while let Some((position, orientation, score, positions)) = queue.pop_front() {
            if let Some(&lowest_score) = lowest_scores.get(&(position, orientation)) {
                if score > lowest_score {
                    continue;
                }
            }

            lowest_scores.insert((position, orientation), score);

            if position == self.ending_position {
                final_paths.push((score, positions));
                continue;
            }

            for next_orientation in Orientation::values() {
                let (dx, dy) = next_orientation.delta();
                let next_position = position.add(dx, dy);
                if self.walls.contains(&next_position) {
                    continue;
                }

                let next_score =
                    score + orientation.quarter_rotations(&next_orientation) * 1000 + 1;

                let mut next_positions = positions.clone();
                next_positions.push(next_position);
                queue.push_back((next_position, next_orientation, next_score, next_positions));
            }
        }

        let min_score = final_paths
            .iter()
            .map(|(score, _)| score)
            .min()
            .copied()
            .expect("Could not reach end");

        let positions = final_paths
            .iter()
            .filter_map(|(score, positions)| {
                if score == &min_score {
                    Some(positions)
                } else {
                    None
                }
            })
            .flatten()
            .collect::<HashSet<_>>();

        (min_score, positions.len())
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut starting_position = None;
        let mut ending_position = None;
        let mut walls = HashSet::new();

        for (y, line) in value.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        walls.insert(Position::new(x, y));
                    }
                    'S' => starting_position = Some(Position::new(x, y)),
                    'E' => ending_position = Some(Position::new(x, y)),
                    '.' => {}
                    char => unreachable!("Unsupported character {char}"),
                };
            }
        }

        Self {
            walls,
            starting_position: starting_position.expect("Could not determine starting position"),
            ending_position: ending_position.expect("Could not determine ending position"),
        }
    }
}
