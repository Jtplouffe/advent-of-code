use std::collections::{HashMap, VecDeque};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{direction::Direction, position::Position, tile::Tile};

pub struct Contraption {
    grid: HashMap<Position, Tile>,
    width: usize,
    height: usize,
}

impl Contraption {
    pub fn energized_tile_count(
        &self,
        starting_position: Position,
        starting_direction: Direction,
    ) -> usize {
        let mut energized_tiles = HashMap::<Position, Vec<Direction>>::new();

        let mut queue = VecDeque::<(Position, Direction)>::new();
        queue.push_back((starting_position, starting_direction));

        while let Some((position, direction)) = queue.pop_front() {
            if !self.is_position_in_grid_bounds(position) {
                continue;
            }

            if let Some(directions) = energized_tiles.get(&position) {
                if directions.contains(&direction) {
                    continue;
                }
            }

            energized_tiles.entry(position).or_default().push(direction);

            match self.grid.get(&position) {
                None => queue.push_back((direction.next_position(position), direction)),
                Some(Tile::Mirror(mirror)) => {
                    queue.push_back(mirror.next_directed_position(position, direction))
                }
                Some(Tile::Splitter(splitter)) => {
                    if splitter.is_vertical() == direction.is_vertical() {
                        queue.push_back((direction.next_position(position), direction));
                    } else {
                        let (first_next_directed_position, second_next_directed_position) =
                            splitter.next_positions(position);
                        queue.push_back(first_next_directed_position);
                        queue.push_back(second_next_directed_position);
                    }
                }
            }
        }

        energized_tiles.len()
    }

    pub fn max_energized_tiles_from_any_starting_point(&self) -> usize {
        let mut starting_points = Vec::with_capacity(self.width * 2 + self.height * 2);
        for x in 0..self.width as isize {
            starting_points.push(((x, 0), Direction::Bottom));
            starting_points.push(((x, self.height as isize - 1), Direction::Top));
        }

        for y in 0..self.height as isize {
            starting_points.push(((0, y), Direction::Right));
            starting_points.push(((self.width as isize - 1, y), Direction::Left));
        }

        starting_points
            .into_par_iter()
            .map(|(starting_position, starting_direction)| {
                self.energized_tile_count(starting_position, starting_direction)
            })
            .max()
            .unwrap()
    }

    fn is_position_in_grid_bounds(&self, (x, y): Position) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let grid = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '.' => None,
                        char => Some(((x as isize, y as isize), Tile::from(char))),
                    })
            })
            .collect();

        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        Self {
            grid,
            width,
            height,
        }
    }
}
