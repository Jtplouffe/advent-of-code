use crate::direction::Direction;
use crate::action::Action;

pub struct Boat {
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
    direction: Direction,
}

impl Boat {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: -1,
            direction: Direction::Right,
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.waypoint_x = 10;
        self.waypoint_y = -1;
        self.direction = Direction::Right;
    }

    pub fn navigate_part_one(&mut self, actions: &Vec<Action>) {
        for action in actions {
            match action {
                Action::North(v) => self.y -= v,
                Action::South(v) => self.y += v,
                Action::East(v) => self.x += v,
                Action::West(v) => self.x -= v,
                Action::Left(v) => self.direction = self.direction.rotate(-(*v as i16)),
                Action::Right(v) => self.direction = self.direction.rotate(*v as i16),
                Action::Forward(v) => match self.direction {
                    Direction::Right => self.x += v,
                    Direction::Bottom => self.y += v,
                    Direction::Left => self.x -= v,
                    Direction::Top => self.y -= v,
                }
            }
        }
    }

    pub fn navigate_part_two(&mut self, actions: &Vec<Action>) {
        for action in actions {
            match action {
                Action::North(v) => self.waypoint_y -= v,
                Action::South(v) => self.waypoint_y += v,
                Action::East(v) => self.waypoint_x += v,
                Action::West(v) => self.waypoint_x -= v,
                Action::Left(v) => for _ in 0..v / 90 % 4 {
                    let temp_x = self.waypoint_x;
                    self.waypoint_x = self.waypoint_y;
                    self.waypoint_y = -temp_x;
                }
                Action::Right(v) => for _ in 0..v / 90 % 4 {
                    let temp_y = self.waypoint_y;
                    self.waypoint_y = self.waypoint_x;
                    self.waypoint_x = -temp_y;
                }
                Action::Forward(v) => {
                    self.y += *v as i32 * self.waypoint_y;
                    self.x += *v as i32 * self.waypoint_x;
                }
            }
        }
    }

    pub fn manhattan_distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as _
    }
}
