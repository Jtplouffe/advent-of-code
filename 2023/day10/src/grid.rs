use crate::{direction::Direction, tile::Tile};

type TilePosition = usize;

pub struct Grid {
    tiles: Vec<Tile>,
    width: usize,
}

impl Grid {
    pub fn max_length_from_start(&self) -> usize {
        let pipe_loop_tiles = self.pipe_loop_tiles();

        (pipe_loop_tiles.len() as f32 / 2.0).ceil() as usize
    }

    fn pipe_loop_tiles(&self) -> Vec<TilePosition> {
        let starting_position = self.starting_position();
        let mut pipe_positions = vec![starting_position];

        let starting_connected_pipes = self.connected_pipes(starting_position);
        let (mut pipe_position, mut pipe_position_from_direction) = starting_connected_pipes[0];
        pipe_position_from_direction = pipe_position_from_direction.opposite();

        loop {
            match &self.tiles[pipe_position] {
                Tile::Pipe(pipe) => {
                    pipe_positions.push(pipe_position);
                    let to_direction = pipe.other_direction(pipe_position_from_direction);
                    (pipe_position, pipe_position_from_direction) = (
                        self.direction_position(pipe_position, to_direction)
                            .unwrap(),
                        to_direction.opposite(),
                    );
                }
                Tile::Start => break,
                Tile::Ground => unreachable!(),
            };
        }

        pipe_positions
    }

    fn starting_position(&self) -> TilePosition {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(index, tile)| match tile {
                Tile::Start => Some(index),
                _ => None,
            })
            .unwrap()
    }

    fn connected_pipes(&self, position: TilePosition) -> Vec<(TilePosition, Direction)> {
        let mut pipes = Vec::new();

        if let Some(north_position) = self.north_position(position) {
            match &self.tiles[north_position] {
                Tile::Pipe(pipe) => {
                    if pipe.connects_to(Direction::South) {
                        pipes.push((north_position, Direction::North));
                    }
                }
                Tile::Start => {
                    pipes.push((north_position, Direction::North));
                }
                _ => {}
            };
        }

        if let Some(south_position) = self.south_position(position) {
            match &self.tiles[south_position] {
                Tile::Pipe(pipe) => {
                    if pipe.connects_to(Direction::North) {
                        pipes.push((south_position, Direction::South));
                    }
                }
                Tile::Start => {
                    pipes.push((south_position, Direction::South));
                }
                _ => {}
            };
        }

        if let Some(east_position) = self.east_position(position) {
            match &self.tiles[east_position] {
                Tile::Pipe(pipe) => {
                    if pipe.connects_to(Direction::West) {
                        pipes.push((east_position, Direction::East));
                    }
                }
                Tile::Start => {
                    pipes.push((east_position, Direction::East));
                }
                _ => {}
            }
        }

        if let Some(west_position) = self.west_position(position) {
            match &self.tiles[west_position] {
                Tile::Pipe(pipe) => {
                    if pipe.connects_to(Direction::East) {
                        pipes.push((west_position, Direction::West));
                    }
                }
                Tile::Start => {
                    pipes.push((west_position, Direction::West));
                }
                _ => {}
            }
        }

        pipes
    }

    fn north_position(&self, from: TilePosition) -> Option<TilePosition> {
        if from < self.width {
            None
        } else {
            Some(from - self.width)
        }
    }

    fn south_position(&self, from: TilePosition) -> Option<TilePosition> {
        if from >= self.tiles.len() - self.width {
            None
        } else {
            Some(from + self.width)
        }
    }

    fn east_position(&self, from: TilePosition) -> Option<TilePosition> {
        if from % self.width >= self.width - 1 {
            None
        } else {
            Some(from + 1)
        }
    }

    fn west_position(&self, from: TilePosition) -> Option<TilePosition> {
        if from % self.width == 0 {
            None
        } else {
            Some(from - 1)
        }
    }

    fn direction_position(&self, from: TilePosition, direction: Direction) -> Option<TilePosition> {
        match direction {
            Direction::North => self.north_position(from),
            Direction::South => self.south_position(from),
            Direction::East => self.east_position(from),
            Direction::West => self.west_position(from),
        }
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let tiles = value
            .chars()
            .filter_map(|char| match char {
                '\n' => None,
                char => Some(Tile::from(char)),
            })
            .collect();

        Self { tiles, width }
    }
}
