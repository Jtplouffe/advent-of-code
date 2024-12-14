use crate::position::{Position, Quadrant};

#[derive(Clone)]
pub struct Robot {
    room_width: usize,
    room_height: usize,
    pub position: Position,
    velocity: Position,
}

impl Robot {
    pub fn new(room_width: usize, room_height: usize, robot_str: &str) -> Self {
        let (initial_position, velocity) = robot_str.split_once(' ').unwrap();

        let (_, initial_position) = initial_position.split_once('=').unwrap();
        let (x, y) = initial_position.split_once(',').unwrap();
        let initial_position = Position::new(x.parse().unwrap(), y.parse().unwrap());

        let (_, velocity) = velocity.split_once('=').unwrap();
        let (x, y) = velocity.split_once(',').unwrap();
        let velocity = Position::new(x.parse().unwrap(), y.parse().unwrap());

        Self {
            room_width,
            room_height,
            position: initial_position,
            velocity,
        }
    }

    pub fn mov(&mut self, seconds: usize) {
        let x = (self.position.x + self.velocity.x * seconds as isize)
            .rem_euclid(self.room_width as isize);
        let y = (self.position.y + self.velocity.y * seconds as isize)
            .rem_euclid(self.room_height as isize);

        self.position = Position::new(x, y);
    }

    pub fn quadrant(&self) -> Quadrant {
        self.position.quadrant(self.room_width, self.room_height)
    }
}
