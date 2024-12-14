use std::collections::{HashMap, HashSet};

use crate::{
    position::{Position, Quadrant},
    robot::Robot,
};

#[derive(Clone)]
pub struct Room {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Room {
    pub fn new(width: usize, height: usize, robots_str: &str) -> Self {
        let robots = robots_str
            .lines()
            .map(|robot_str| Robot::new(width, height, robot_str))
            .collect();

        Self {
            width,
            height,
            robots,
        }
    }

    pub fn process(&mut self, seconds: usize) {
        for robot in &mut self.robots {
            robot.mov(seconds);
        }
    }

    pub fn safety_factory(&self) -> usize {
        let mut quadrant_positions = HashMap::<Quadrant, Vec<&Position>>::new();
        for robot in &self.robots {
            let quadrant = robot.quadrant();
            if quadrant == Quadrant::OnAxis {
                continue;
            }

            quadrant_positions
                .entry(quadrant)
                .or_default()
                .push(&robot.position);
        }

        quadrant_positions
            .values()
            .map(|positions| positions.len())
            .product()
    }

    pub fn seconds_for_first_christmas_tree(&mut self) -> usize {
        'seconds: for seconds in 1.. {
            self.process(1);

            let mut positions = HashSet::new();
            for robot in &self.robots {
                if positions.contains(&robot.position) {
                    continue 'seconds;
                }

                positions.insert(&robot.position);
            }

            self.print();

            return seconds;
        }

        unreachable!()
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position::new(x as isize, y as isize);
                let count = self
                    .robots
                    .iter()
                    .filter(|robot| robot.position == position)
                    .count();

                let str = match count {
                    0 => ".".to_string(),
                    count => count.to_string(),
                };

                print!("{str}");
            }

            println!();
        }
    }
}
