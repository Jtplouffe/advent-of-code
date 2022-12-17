type RockTiles<'a> = &'a [&'a [bool]];

pub const ROCK_TILES: &[RockTiles] = &[
    // ####
    &[&[true, true, true, true]],
    // .#.
    // ###
    // .#.
    &[
        &[false, true, false],
        &[true, true, true],
        &[false, true, false],
    ],
    // ..#
    // ..#
    // ###
    &[
        &[false, false, true],
        &[false, false, true],
        &[true, true, true],
    ],
    // #
    // #
    // #
    // #
    &[&[true], &[true], &[true], &[true]],
    // ##
    // ##
    &[&[true, true], &[true, true]],
];

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Rock {
    tiles: RockTiles<'static>,
    pub position: Position,
}

impl Rock {
    pub fn new(rock_index: usize, bottom_position: Position) -> Self {
        let tiles = ROCK_TILES[rock_index % ROCK_TILES.len()];

        Self {
            tiles,
            position: Position::new(bottom_position.x, bottom_position.y + tiles.len() - 1),
        }
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn bottom_left_position(&self) -> Position {
        Position::new(self.position.x, self.position.y - (self.height() - 1))
    }

    pub fn top_right_position(&self) -> Position {
        Position::new(self.position.x + self.width() - 1, self.position.y)
    }

    pub fn iter_tile_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(move |(y_offset, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x_offset, &occupied)| {
                        if occupied {
                            Some(Position::new(
                                self.position.x + x_offset,
                                self.position.y - y_offset,
                            ))
                        } else {
                            None
                        }
                    })
            })
    }

    pub fn iter_left_tile_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.tiles.iter().enumerate().map(|(y_offset, row)| {
            let (x_offset, _) = row
                .iter()
                .enumerate()
                .find(move |(_, &occupied)| occupied)
                .unwrap();
            Position::new(self.position.x + x_offset, self.position.y - y_offset)
        })
    }

    pub fn iter_right_tile_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.tiles.iter().enumerate().map(|(y_offset, row)| {
            let (x_offset, _) = row
                .iter()
                .enumerate()
                .rev()
                .find(move |(_, &occupied)| occupied)
                .unwrap();
            Position::new(self.position.x + x_offset, self.position.y - y_offset)
        })
    }

    pub fn iter_bottom_tile_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.width()).map(|x_offset| {
            let y_offset = (0..self.height())
                .rev()
                .find(|&y| self.tiles[y][x_offset])
                .unwrap();

            Position::new(self.position.x + x_offset, self.position.y - y_offset)
        })
    }
}
