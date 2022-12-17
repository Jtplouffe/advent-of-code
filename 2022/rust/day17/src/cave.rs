use std::collections::HashSet;

use crate::movement::Movement;
use crate::rock::{Position, Rock};

pub struct Grid<'a> {
    movements: &'a [Movement],
    rock_tiles_columns: Vec<HashSet<usize>>,
    highest_occupied_tile: isize,
    next_movement_index: usize,
    rock_count: usize,
}

impl<'a> Grid<'a> {
    pub fn new(movements: &'a [Movement]) -> Self {
        Self {
            movements,
            rock_tiles_columns: vec![HashSet::new(); 7],
            highest_occupied_tile: -1,
            next_movement_index: 0,
            rock_count: 0,
        }
    }

    pub fn height(&self) -> usize {
        (self.highest_occupied_tile + 1) as usize
    }

    pub fn generate_rocks(&mut self, rock_count: usize) {
        for _ in 0..rock_count {
            self.generate_rock();
        }
    }

    pub fn generate_rock(&mut self) {
        let mut rock = Rock::new(
            self.rock_count,
            Position::new(2, (self.highest_occupied_tile + 4) as usize),
        );

        loop {
            self.handle_rock_movement(&mut rock);

            if !self.can_rock_fall(&rock) {
                break;
            }

            rock.position.y -= 1;
        }

        if rock.position.y as isize > self.highest_occupied_tile {
            self.highest_occupied_tile = rock.position.y as isize;
        }

        for tile_position in rock.iter_tile_positions() {
            self.rock_tiles_columns[tile_position.x].insert(tile_position.y);
        }

        self.rock_count += 1;
    }

    fn handle_rock_movement(&mut self, rock: &mut Rock) {
        let movement = self.next_movement();

        match movement {
            Movement::Left => {
                if rock.position.x == 0 {
                    return;
                }

                let can_move = rock
                    .iter_left_tile_positions()
                    .all(|position| !self.rock_tiles_columns[position.x - 1].contains(&position.y));

                if can_move {
                    rock.position.x -= 1;
                }
            }
            Movement::Right => {
                if rock.top_right_position().x == self.rock_tiles_columns.len() - 1 {
                    return;
                }

                let can_move = rock
                    .iter_right_tile_positions()
                    .all(|position| !self.rock_tiles_columns[position.x + 1].contains(&position.y));

                if can_move {
                    rock.position.x += 1;
                }
            }
        };
    }

    fn next_movement(&mut self) -> &Movement {
        let movement = &self.movements[self.next_movement_index % self.movements.len()];
        self.next_movement_index += 1;

        movement
    }

    fn can_rock_fall(&self, rock: &Rock) -> bool {
        if rock.bottom_left_position().y == 0 {
            return false;
        }

        rock.iter_bottom_tile_positions()
            .all(|position| !self.rock_tiles_columns[position.x].contains(&(position.y - 1)))
    }
}
