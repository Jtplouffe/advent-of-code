use std::collections::{HashMap, HashSet};

use crate::{garden::Garden, position::Position};

pub struct Field {
    gardens: Vec<Garden>,
}

impl Field {
    pub fn total_fence_price(&self) -> usize {
        self.gardens.iter().map(|garden| garden.fence_price()).sum()
    }

    pub fn total_fence_bulk_price(&self) -> usize {
        self.gardens.iter().map(|garden| garden.bulk_fence_price()).sum()
    }

    fn extract_garden(
        char: char,
        current_tile: &Position,
        current_garden: &mut HashSet<Position>,
        grid: &HashMap<Position, char>,
        processed: &mut HashSet<Position>,
    ) {
        if processed.contains(current_tile) || grid.get(current_tile) != Some(&char) {
            return;
        }

        processed.insert(current_tile.clone());
        current_garden.insert(current_tile.clone());

        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            if (current_tile.x == 0 && dx == -1) || (current_tile.y == 0 && dy == -1) {
                continue;
            }

            let next_tile = current_tile.add(dx, dy);
            Self::extract_garden(char, &next_tile, current_garden, grid, processed);
        }
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();

        let mut gardens = Vec::new();
        let mut processed = HashSet::new();

        let mut grid = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let tile = Position::new(x, y);
                grid.insert(tile, char);
            }
        }

        let mut current_garden = HashSet::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let tile = Position::new(x, y);
                Self::extract_garden(char, &tile, &mut current_garden, &grid, &mut processed);

                if !current_garden.is_empty() {
                    gardens.push(Garden::new(char, current_garden));
                    current_garden = HashSet::new();
                }
            }
        }

        Self { gardens }
    }
}
