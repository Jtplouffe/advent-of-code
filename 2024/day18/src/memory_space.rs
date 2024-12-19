use std::collections::{HashMap, HashSet, VecDeque};

use crate::position::Position;

const DELTAS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct MemorySpace {
    width: usize,
    height: usize,
    walls_at_times: Vec<HashSet<Position>>,
}

impl MemorySpace {
    pub fn new(width: usize, height: usize, walls: &str) -> Self {
        let mut walls_at_times: Vec<HashSet<Position>> = vec![HashSet::new()];
        for line in walls.lines() {
            let (x, y) = line.split_once(',').expect("Invalid wall coordinate");
            let x = x.parse().expect("Invalid x position");
            let y = y.parse().expect("Invalid x position");

            let mut walls = walls_at_times.last().unwrap().clone();

            walls.insert(Position::new(x, y));
            walls_at_times.push(walls);
        }

        Self {
            width,
            height,
            walls_at_times,
        }
    }

    pub fn shortest_distance_at_time(&self, time: usize) -> usize {
        let walls = self
            .walls_at_times
            .get(time)
            .unwrap_or(self.walls_at_times.last().unwrap());

        self.traverse(walls).expect("Could not solve maze")
    }

    pub fn first_byte_to_block_exit(&self) -> String {
        // This could be binary searched instead
        let blocked_at_time = self
            .walls_at_times
            .iter()
            .position(|walls| self.traverse(walls).is_none())
            .expect("Exit is never blocked");

        let walls = &self.walls_at_times[blocked_at_time];
        let previous_walls = &self.walls_at_times[blocked_at_time - 1];

        let position = walls
            .iter()
            .find(|position| !previous_walls.contains(position))
            .expect("Current walls does not have more entry that the previous");
        format!("{},{}", position.x, position.y)
    }

    fn traverse(&self, walls: &HashSet<Position>) -> Option<usize> {
        let mut lowest_distances = HashMap::<Position, usize>::new();

        let mut queue = VecDeque::new();
        queue.push_back((Position::new(0, 0), 0));

        while let Some((position, distance)) = queue.pop_front() {
            if walls.contains(&position) {
                continue;
            }

            if let Some(&lowest_distance) = lowest_distances.get(&position) {
                if distance >= lowest_distance {
                    continue;
                }
            }

            lowest_distances.insert(position.clone(), distance);

            if self.is_ending_position(&position) {
                continue;
            }

            for (dx, dy) in DELTAS {
                // Bound check
                if (dx == -1 && position.x == 0)
                    || (dy == -1 && position.y == 0)
                    || (dx == 1 && position.x == self.width - 1)
                    || (dy == 1 && position.y == self.height - 1)
                {
                    continue;
                }

                let next_position = position.add(dx, dy);
                queue.push_back((next_position, distance + 1));
            }
        }

        lowest_distances
            .get(&Position::new(self.width - 1, self.height - 1))
            .copied()
    }

    #[inline(always)]
    fn is_ending_position(&self, position: &Position) -> bool {
        position.x == self.width - 1 && position.y == self.height - 1
    }
}
