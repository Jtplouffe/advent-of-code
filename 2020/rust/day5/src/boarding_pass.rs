use std::str::FromStr;
use std::collections::VecDeque;

enum Direction {
    Front,
    Back,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, ()> {
        match c {
            'F' => Ok(Self::Front),
            'B' => Ok(Self::Back),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

pub struct BoardingPass {
    pub row: u8,
    pub column: u8,
    pub id: u32,
}

impl FromStr for BoardingPass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions: VecDeque<Direction> = s.as_bytes().iter().map(|d| Direction::from_char(*d as char).unwrap()).collect();

        let mut row_range: (u8, u8) = (0, 127);
        let mut column_range: (u8, u8) = (0, 7);

        while row_range.0 != row_range.1 || column_range.0 != column_range.1 {
            match directions.pop_front().unwrap() {
                Direction::Front => row_range.1 = (row_range.0 + row_range.1) / 2,
                Direction::Back => row_range.0 = (row_range.0 + row_range.1) / 2 + 1,
                Direction::Left => column_range.1 = (column_range.0 + column_range.1) / 2,
                Direction::Right => column_range.0 = (column_range.0 + column_range.1) / 2 + 1,
            }
        }

        Ok(Self { row: row_range.0, column: column_range.0, id: row_range.0 as u32 * 8 + column_range.0 as u32 })
    }
}