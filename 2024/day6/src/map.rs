use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::position::{Direction, Position};

pub struct Map {
    width: usize,
    height: usize,
    obstructions: HashSet<Position>,
    initial_guard_position: Position,
}

impl Map {
    pub fn guard_positions(&self) -> HashSet<Position> {
        let (width, height) = (self.width as isize, self.height as isize);

        let mut positions = HashSet::new();
        let mut position = self.initial_guard_position;
        let mut direction = Direction::North;

        loop {
            positions.insert(position);

            let next_position = position.mov(&direction);

            if next_position.x < 0
                || next_position.x >= width
                || next_position.y < 0
                || next_position.y >= height
            {
                break;
            }

            if self.obstructions.contains(&next_position) {
                direction = direction.turn_right();
            } else {
                position = next_position;
            }
        }

        positions
    }

    pub fn possible_looping_obstructions(&self) -> HashSet<Position> {
        let mut guard_positions = self.guard_positions();
        guard_positions.remove(&self.initial_guard_position);

        guard_positions
            .into_par_iter()
            .filter(|position| self.will_obstruction_causes_loop(position))
            .collect()
    }

    fn will_obstruction_causes_loop(&self, new_obstruction: &Position) -> bool {
        let (width, height) = (self.width as isize, self.height as isize);

        let mut oriented_positions = HashSet::<(Position, Direction)>::new();
        let mut position = self.initial_guard_position;
        let mut direction = Direction::North;

        loop {
            let oriented_position = (position, direction);
            if oriented_positions.contains(&oriented_position) {
                return true;
            }

            oriented_positions.insert(oriented_position);

            let next_position = position.mov(&direction);

            if next_position.x < 0
                || next_position.x >= width
                || next_position.y < 0
                || next_position.y >= height
            {
                break;
            }

            if self.obstructions.contains(&next_position) || &next_position == new_obstruction {
                direction = direction.turn_right();
            } else {
                position = next_position;
            }
        }

        false
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut obstructions = HashSet::new();
        let mut guard_position = None;

        let lines: Vec<_> = value.lines().collect();

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        obstructions.insert(Position::new(x as isize, y as isize));
                    }
                    '^' => {
                        guard_position = Some(Position::new(x as isize, y as isize));
                    }
                    _ => {}
                }
            }
        }

        Self {
            obstructions,
            initial_guard_position: guard_position.unwrap(),
            width: lines[0].len(),
            height: lines.len(),
        }
    }
}
