use std::collections::BinaryHeap;
use std::str::FromStr;

use crate::dijkstra_state::DijkstraState;

const START_INDEX: usize = 0;

pub struct Cave {
    width: usize,
    risk_levels: Vec<usize>,
}

impl Cave {
    // https://doc.rust-lang.org/std/collections/binary_heap/index.html
    pub fn lowest_risk_level(&self) -> usize {
        let finish_index = self.risk_levels.len() - 1;

        let mut distances = vec![usize::MAX; self.risk_levels.len()];
        let mut heap = BinaryHeap::<DijkstraState>::new();

        distances[START_INDEX] = 0;
        heap.push(DijkstraState::new(0, START_INDEX));

        while let Some(DijkstraState { distance, index }) = heap.pop() {
            if index == finish_index {
                return distance + self.risk_levels[index] - 1;
            }

            if distance > distances[index] {
                continue;
            }

            for adjacent_index in self.get_adjacent_indexes(index) {
                let next_state = DijkstraState::new(distance + self.risk_levels[index], adjacent_index);

                if next_state.distance < distances[next_state.index] {
                    heap.push(next_state);
                    distances[next_state.index] = next_state.distance;
                }
            }
        }

        unreachable!()
    }

    fn get_adjacent_indexes(&self, index: usize) -> Vec<usize> {
        let mut adjacent = Vec::new();

        if index >= self.width {
            // up
            adjacent.push(index - self.width);
        }
        if index < self.risk_levels.len() - self.width {
            // down
            adjacent.push(index + self.width);
        }
        if index % self.width > 0 {
            // left
            adjacent.push(index - 1);
        }
        if index % self.width < self.width {
            // right
            adjacent.push(index + 1);
        }

        adjacent
    }

    pub fn expand_risk_levels(&mut self, tile_count: usize) {
        let mut new_risk_levels = Vec::new();

        for tile in 0..tile_count {
            for line in self.risk_levels.chunks(self.width) {
                for i in 0..tile_count {
                    for risk_level in line {
                        let risk_level = risk_level + i + tile;

                        if risk_level > 9 {
                            new_risk_levels.push(risk_level % 9);
                        } else {
                            new_risk_levels.push(risk_level);
                        }
                    }
                }
            }
        }

        self.risk_levels = new_risk_levels;
        self.width *= tile_count;
    }
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        let risk_levels: Vec<_> = s.split("").filter_map(|r| usize::from_str(r).ok()).collect();
        let width = s.lines().nth(0).unwrap().len();

        Self {
            width,
            risk_levels,
        }
    }
}
