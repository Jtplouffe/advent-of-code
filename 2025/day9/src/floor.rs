use std::collections::{HashMap, HashSet};

use crate::position::Position;

pub struct Floor {
    red_tiles: Vec<Position>,
}

impl Floor {
    pub fn largest_rectangle(&self) -> usize {
        let mut max_area = 0;

        for (i, first_corner) in self.red_tiles.iter().enumerate() {
            for second_corner in &self.red_tiles[i + 1..] {
                let area = first_corner.area(second_corner);
                if area > max_area {
                    max_area = area
                }
            }
        }

        max_area
    }

    pub fn lagset_rectangle_within_area(&self) -> usize {
        let border_tiles = self.generate_border_tiles();
        let bounds = self.bounds();

        let mut max_area = 0;

        let mut is_in_area_cache = HashMap::new();

        for (i, first_corner) in self.red_tiles.iter().enumerate() {
            let mut second_corners: Vec<_> = self.red_tiles[i + 1..].iter().collect();
            second_corners
                .sort_by_key(|second_corner| first_corner.euclidean_distance(second_corner));

            let mut invalid_second_corners = Vec::<&Position>::new();

            'corners: for second_corner in second_corners {
                // If another second_corner was invalid and was within the bounds of [first_corner, second_corner],
                // the current second_corner must therefore also be invalid, as it wraps entirely the other invaild corner.
                let wraps_other_invaild_corner =
                    invalid_second_corners.iter().any(|invalid_second_corner| {
                        first_corner.is_in_area(second_corner, invalid_second_corner)
                    });
                if wraps_other_invaild_corner {
                    continue;
                }

                for area_position in first_corner.iter_area_position(second_corner) {
                    if !Self::is_in_area(
                        area_position,
                        &border_tiles,
                        &bounds,
                        &mut is_in_area_cache,
                    ) {
                        invalid_second_corners.push(second_corner);
                        continue 'corners;
                    }
                }

                let area = first_corner.area(second_corner);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }

    pub fn generate_border_tiles(&self) -> HashSet<Position> {
        let mut border_tiles: HashSet<_> = self.red_tiles.iter().map(ToOwned::to_owned).collect();

        for (first_red_tile_index, first_red_tile) in self.red_tiles.iter().enumerate() {
            let second_red_tile_index = (first_red_tile_index + 1) % self.red_tiles.len();
            let second_red_tile = &self.red_tiles[second_red_tile_index];
            let (dx, dy) = first_red_tile.unit_direction_delta(second_red_tile);

            let mut current_position = first_red_tile.add(dx, dy);
            while &current_position != second_red_tile {
                let next_current_position = current_position.add(dx, dy);
                border_tiles.insert(current_position);
                current_position = next_current_position;
            }
        }

        border_tiles
    }

    fn is_in_area(
        position: Position,
        border_tiles: &HashSet<Position>,
        (min, max): &(Position, Position),
        is_in_area_cache: &mut HashMap<Position, bool>,
    ) -> bool {
        if let Some(&cached_result) = is_in_area_cache.get(&position) {
            return cached_result;
        }

        // The border is considered in the area
        if border_tiles.contains(&position) {
            is_in_area_cache.insert(position, true);
            return true;
        }

        let (dx, dy) = position.unit_direction_delta(&Position::new(position.x, min.y - 1));

        let mut current_position = position.add(dx, dy);
        let mut border_crossing_count = 0;

        while current_position.x >= min.x
            && current_position.x <= max.x
            && current_position.y >= min.y
            && current_position.y <= max.y
        {
            if let Some(&is_in_area) = is_in_area_cache.get(&current_position) {
                let result = if border_crossing_count % 2 == 0 {
                    is_in_area
                } else {
                    !is_in_area
                };
                is_in_area_cache.insert(position, is_in_area);
                return result;
            }

            if border_tiles.contains(&current_position) {
                border_crossing_count += 1;
            }

            current_position = current_position.add(dx, dy);
        }

        let result = border_crossing_count % 2 == 1;
        is_in_area_cache.insert(position, result);

        result
    }

    fn bounds(&self) -> (Position, Position) {
        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;

        for &Position { x, y } in &self.red_tiles {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }

            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        (Position::new(min_x, min_y), Position::new(max_x, max_y))
    }
}

impl From<&str> for Floor {
    fn from(value: &str) -> Self {
        let red_tiles = value.lines().map(Position::from).collect();

        Self { red_tiles }
    }
}
