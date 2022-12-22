use std::collections::HashMap;

use crate::{
    cell::{Cell, Direction, Position},
    movement::Movement,
};

pub struct Board {
    current: (Position, Direction),
    cells: HashMap<Position, Cell>,
}

impl Board {
    pub fn final_password(&mut self, movements: &[Movement]) -> usize {
        for movement in movements {
            self.apply_movement(movement);
        }

        (self.current.0.y as usize + 1) * 1000
            + (self.current.0.x as usize + 1) * 4
            + self.current.1.rotatory_index()
    }

    fn apply_movement(&mut self, movement: &Movement) {
        match movement {
            Movement::Move(value) => self.move_in_current_direction(*value),
            Movement::Rotate(rotation) => self.current.1 = self.current.1.with_rotation(rotation),
        }
    }

    fn move_in_current_direction(&mut self, value: usize) {
        for _ in 0..value {
            let next_position = self.current.0.with_move_in_direction(&self.current.1);
            match self.cells.get(&next_position) {
                Some(cell) => match cell {
                    Cell::Open => self.current.0 = next_position,
                    Cell::Wall => return,
                },
                None => {
                    let wrapped_position = self.wrapped_current_position();

                    match self.cells[&wrapped_position] {
                        Cell::Open => self.current.0 = wrapped_position,
                        Cell::Wall => return,
                    };
                }
            };
        }
    }

    fn wrapped_current_position(&self) -> Position {
        let (current_x, current_y) = (self.current.0.x, self.current.0.y);

        match self.current.1 {
            Direction::North => {
                for y in current_y + 1.. {
                    if !self.cells.contains_key(&Position::new(current_x, y)) {
                        return Position::new(current_x, y - 1);
                    }
                }

                unreachable!()
            }
            Direction::South => {
                for y in (-1..current_y as isize).rev() {
                    if y < 0 || !self.cells.contains_key(&Position::new(current_x, y)) {
                        return Position::new(current_x, y + 1);
                    }
                }

                unreachable!()
            }
            Direction::Est => {
                for x in (-1..current_x as isize).rev() {
                    if x < 0 || !self.cells.contains_key(&Position::new(x, current_y)) {
                        return Position::new(x + 1, current_y);
                    }
                }

                unreachable!()
            }
            Direction::West => {
                for x in current_x.. {
                    if !self.cells.contains_key(&Position::new(x, current_y)) {
                        return Position::new(x - 1, current_y);
                    }
                }

                unreachable!()
            }
        }
    }
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let cells = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| {
                    Cell::try_from(c)
                        .and_then(|cell| Ok((Position::new(x as isize, y as isize), cell)))
                })
            })
            .collect::<HashMap<_, _>>();

        let initial_position = *cells
            .keys()
            .filter(|position| position.y == 0)
            .reduce(|lowest, current| {
                if current.x < lowest.x {
                    current
                } else {
                    lowest
                }
            })
            .unwrap();

        Self {
            current: (initial_position, Direction::Est),
            cells,
        }
    }
}
