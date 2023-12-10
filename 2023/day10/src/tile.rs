use crate::pipe::Pipe;

pub enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ground,
            'S' => Self::Start,
            c => Self::Pipe(Pipe::from(c)),
        }
    }
}
