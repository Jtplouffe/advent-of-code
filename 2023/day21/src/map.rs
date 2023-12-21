use std::collections::{HashSet, VecDeque};

type Position = (usize, usize);

pub struct Map {
    rocks: HashSet<Position>,
    starting_position: Position,
    width: usize,
    height: usize,
}

impl Map {
    pub fn reached_garden_count_after_steps(&self, step_count: usize) -> usize {
        let mut visited_positions_at_steps = HashSet::<(Position, usize)>::new();
        let mut queue = VecDeque::<(Position, usize)>::new();
        queue.push_back((self.starting_position, 0));

        while let Some((position, steps)) = queue.pop_front() {
            if steps > step_count {
                continue;
            }

            let visited_position_at_steps_key = (position, steps);
            if visited_positions_at_steps.contains(&visited_position_at_steps_key) {
                continue;
            }

            visited_positions_at_steps.insert(visited_position_at_steps_key);

            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] as [(isize, isize); 4] {
                if (dx == -1 && position.0 == 0)
                    || (dy == -1 && position.1 == 0)
                    || (dx == 1 && position.0 == self.width - 1)
                    || (dy == 1 && position.1 == self.height - 1)
                {
                    continue;
                }

                let next_position = (
                    (position.0 as isize + dx) as usize,
                    (position.1 as isize + dy) as usize,
                );
                if self.rocks.contains(&next_position) {
                    continue;
                }

                queue.push_back((next_position, steps + 1));
            }
        }

        visited_positions_at_steps
            .iter()
            .filter(|(_, steps)| *steps == step_count)
            .count()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let rocks = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '#' => Some((x, y)),
                        _ => None,
                    })
            })
            .collect();
        let starting_position = value
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars().enumerate().find_map(|(x, char)| match char {
                    'S' => Some((x, y)),
                    _ => None,
                })
            })
            .unwrap();

        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        Self {
            rocks,
            starting_position,
            width,
            height,
        }
    }
}
