use std::collections::HashSet;

use crate::position::Position;

pub struct Garden {
    pub char: char,
    pub tiles: HashSet<Position>,
}

impl Garden {
    pub fn new(char: char, tiles: HashSet<Position>) -> Self {
        Self { char, tiles }
    }

    pub fn fence_price(&self) -> usize {
        self.perimiter() * self.area()
    }

    pub fn bulk_fence_price(&self) -> usize {
        self.side_count() * self.area()
    }

    fn perimiter(&self) -> usize {
        let Position { x: min_x, y: min_y } = self.min_position();
        let Position { x: max_x, y: max_y } = self.max_position();

        let mut perimiter = 0;
        for y in min_y..=max_y + 1 {
            let mut previous_position_was_tile = false;

            for x in min_x..=max_x + 1 {
                if self.tiles.contains(&Position::new(x, y)) {
                    if !previous_position_was_tile {
                        perimiter += 1;
                    }

                    previous_position_was_tile = true;
                } else if previous_position_was_tile {
                    perimiter += 1;
                    previous_position_was_tile = false;
                }
            }
        }

        for x in min_x..=max_x + 1 {
            let mut previous_position_was_tile = false;

            for y in min_y..=max_y + 1 {
                if self.tiles.contains(&Position::new(x, y)) {
                    if !previous_position_was_tile {
                        perimiter += 1;
                    }

                    previous_position_was_tile = true;
                } else if previous_position_was_tile {
                    perimiter += 1;
                    previous_position_was_tile = false;
                }
            }
        }

        perimiter
    }

    fn side_count(&self) -> usize {
        let Position { x: min_x, y: min_y } = self.min_position();
        let Position { x: max_x, y: max_y } = self.max_position();

        let mut side_count = 0;

        for y in min_y..=max_y + 1 {
            let mut previous_tile_edge = None;

            for x in min_x..=max_x + 1 {
                let is_top_tile_empty = y == 0 || !self.tiles.contains(&Position::new(x, y - 1));
                let is_tile_empty = !self.tiles.contains(&Position::new(x, y));

                let is_edge = is_top_tile_empty != is_tile_empty;
                if (!is_edge && previous_tile_edge.is_some())
                    || (is_edge && previous_tile_edge == Some(is_tile_empty))
                {
                    side_count += 1;
                }

                if is_edge {
                    previous_tile_edge = Some(is_top_tile_empty);
                } else {
                    previous_tile_edge = None;
                }
            }
        }

        for x in min_x..=max_x + 1 {
            let mut previous_tile_edge = None;

            for y in min_y..=max_y + 1 {
                let is_left_tile_empty = x == 0 || !self.tiles.contains(&Position::new(x - 1, y));
                let is_tile_empty = !self.tiles.contains(&Position::new(x, y));

                let is_edge = is_left_tile_empty != is_tile_empty;
                if (!is_edge && previous_tile_edge.is_some())
                    || (is_edge && previous_tile_edge == Some(is_tile_empty))
                {
                    side_count += 1;
                }

                if is_edge {
                    previous_tile_edge = Some(is_left_tile_empty);
                } else {
                    previous_tile_edge = None;
                }
            }
        }

        side_count
    }

    fn area(&self) -> usize {
        self.tiles.len()
    }

    fn min_position(&self) -> Position {
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;

        for &Position { x, y } in &self.tiles {
            if x < min_x {
                min_x = x
            }

            if y < min_y {
                min_y = y
            }
        }

        Position::new(min_x, min_y)
    }

    fn max_position(&self) -> Position {
        let mut max_x = usize::MIN;
        let mut max_y = usize::MIN;

        for &Position { x, y } in &self.tiles {
            if x > max_x {
                max_x = x
            }

            if y > max_y {
                max_y = y
            }
        }

        Position::new(max_x, max_y)
    }
}
