use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::direction::Direction;

pub struct Grid {
    grid: Vec<u32>,
    width: usize,
}

impl Grid {
    pub fn min_heat_loss(&self, minimum_straight_line: usize, maximum_straight_line: usize) -> u32 {
        let mut min_weights = HashMap::<(usize, Direction), u32>::new();
        let mut queue = BinaryHeap::<Reverse<(u32, usize, Option<Direction>)>>::new();
        queue.push(Reverse((0, 0, None)));

        while let Some(Reverse((weight, position, direction))) = queue.pop() {
            if position == self.grid.len() - 1 {
                return weight;
            }

            if let Some(direction) = direction {
                if min_weights
                    .get(&(position, direction))
                    .is_some_and(|&w| w < weight)
                {
                    continue;
                }
            }

            for next_direction in Direction::values() {
                if direction.is_some_and(|direction| {
                    direction.is_vertical() == next_direction.is_vertical()
                }) {
                    continue;
                }

                let mut next_weight = weight;
                for i in 1..=maximum_straight_line {
                    let next_position = match next_direction {
                        Direction::Top => {
                            let sub = self.width * i;
                            if sub > position {
                                continue;
                            }

                            position - sub
                        }
                        Direction::Bottom => {
                            let add = self.width * i;
                            if add + position >= self.grid.len() {
                                continue;
                            }

                            position + add
                        }
                        Direction::Left => {
                            let x = position % self.width;
                            if i > x {
                                continue;
                            }

                            position - i
                        }
                        Direction::Right => {
                            let x = position % self.width;
                            if x + i >= self.width {
                                continue;
                            }

                            position + i
                        }
                    };

                    next_weight += self.grid[next_position];
                    if i < minimum_straight_line {
                        continue;
                    }

                    if next_weight
                        < *min_weights
                            .get(&(next_position, next_direction))
                            .unwrap_or(&u32::MAX)
                    {
                        min_weights.insert((next_position, next_direction), next_weight);
                        queue.push(Reverse((next_weight, next_position, Some(next_direction))));
                    }
                }
            }
        }

        unreachable!()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let grid = value
            .replace('\n', "")
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .collect();
        let width = value.lines().next().unwrap().len();

        Self { grid, width }
    }
}
