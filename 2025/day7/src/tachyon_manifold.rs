use std::collections::{HashMap, HashSet, VecDeque};

use crate::position::Position;

pub struct TachyonManifold {
    splitter_positions: HashSet<Position>,
    start_position: Position,
}

impl TachyonManifold {
    pub fn split_count(&self) -> u32 {
        let mut split_count = 0;

        let max_splitter_position = self.max_splitter_position();

        let mut visited_positions = HashSet::<Position>::new();
        let mut queue = VecDeque::<Position>::new();
        queue.push_back(self.start_position);

        while let Some(current_position) = queue.pop_front() {
            if visited_positions.contains(&current_position) {
                continue;
            }

            visited_positions.insert(current_position);

            // Beam is currently on a splitter
            if self.splitter_positions.contains(&current_position) {
                let left = current_position.add(-1, 0);
                let right = current_position.add(1, 0);

                queue.push_back(left);
                queue.push_back(right);

                split_count += 1;
                continue;
            }

            // No more splitters will be met further down
            if current_position.y >= max_splitter_position.y {
                continue;
            }

            // Beam moves down by 1
            queue.push_back(current_position.add(0, 1));
        }

        split_count
    }

    pub fn split_count_quantum(&self) -> u64 {
        let max_splitter_position = self.max_splitter_position();

        let mut cache = HashMap::new();
        self.split_count_quantum_inner(self.start_position, max_splitter_position, &mut cache);
        *cache.get(&self.start_position).unwrap() + 1
    }

    fn split_count_quantum_inner(
        &self,
        current_position: Position,
        max_splitter_position: &Position,
        cache: &mut HashMap<Position, u64>,
    ) {
        if cache.contains_key(&current_position) {
            return;
        }

        if self.splitter_positions.contains(&current_position) {
            let left = current_position.add(-1, 0);
            let right = current_position.add(1, 0);

            self.split_count_quantum_inner(left, max_splitter_position, cache);
            self.split_count_quantum_inner(right, max_splitter_position, cache);

            let left_split_count = cache.get(&left).unwrap();
            let right_split_count = cache.get(&right).unwrap();
            cache.insert(current_position, left_split_count + right_split_count + 1);

            return;
        }

        // End reached
        if current_position.y >= max_splitter_position.y {
            cache.insert(current_position, 0);
            return;
        }

        let child_position = current_position.add(0, 1);
        self.split_count_quantum_inner(child_position, max_splitter_position, cache);

        let child_split_count = cache.get(&child_position).copied().unwrap_or(0);
        cache.insert(current_position, child_split_count);
    }

    fn max_splitter_position(&self) -> &Position {
        self.splitter_positions
            .iter()
            .max_by(|a, b| a.y.cmp(&b.y))
            .expect("No splitters")
    }
}

impl From<&str> for TachyonManifold {
    fn from(value: &str) -> Self {
        let mut splitter_positions = HashSet::new();
        let mut start_position: Option<Position> = None;

        for (y, line) in value.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    'S' => start_position = Some(Position::new(x as isize, y as isize)),
                    '^' => {
                        splitter_positions.insert(Position::new(x as isize, y as isize));
                    }
                    _ => {}
                }
            }
        }

        Self {
            splitter_positions,
            start_position: start_position.expect("Could not find start position"),
        }
    }
}
