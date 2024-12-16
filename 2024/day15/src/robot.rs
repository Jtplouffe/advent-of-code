use crate::position::Position;

#[derive(Clone)]
pub enum RobotMovement {
    Up,
    Right,
    Down,
    Left,
}

impl RobotMovement {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

impl From<char> for RobotMovement {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Robot {
    pub position: Position,
    pub movements: Vec<RobotMovement>,
}

impl Robot {
    pub fn new(position: Position, movements: Vec<RobotMovement>) -> Self {
        Self {
            position,
            movements,
        }
    }
}
