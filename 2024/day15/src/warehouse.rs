use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};

use crate::{
    position::Position,
    robot::{Robot, RobotMovement},
    tile::Tile,
};

#[derive(Clone)]
pub struct Warehouse {
    grid: HashMap<Position, Tile>,
    robot: Robot,
}

impl Warehouse {
    pub fn perform_robot_movements(&mut self) {
        for movement in &self.robot.movements.clone() {
            self.perform_robot_movement(movement);
        }
    }

    pub fn double_width(&mut self) {
        let (max_x, max_y) = self.grid_max_position();

        let mut new_grid = HashMap::new();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let tile = match self.grid.get(&Position::new(x, y)) {
                    Some(tile) => tile,
                    None => continue,
                };

                let new_position = Position::new(x * 2, y);
                match tile {
                    Tile::Wall => {
                        new_grid.insert(new_position.clone(), Tile::Wall);
                        new_grid.insert(new_position.add(1, 0), Tile::Wall);
                    }
                    Tile::SmallBox => {
                        new_grid.insert(new_position.clone(), Tile::BigBoxFirstHalf);
                        new_grid.insert(new_position.add(1, 0), Tile::BigBoxSecondHalf);
                    }
                    Tile::BigBoxFirstHalf | Tile::BigBoxSecondHalf => {
                        unreachable!("Should not contain big boxes")
                    }
                }
            }
        }

        self.grid = new_grid;
        self.robot.position = Position::new(self.robot.position.x * 2, self.robot.position.y);
    }

    fn perform_robot_movement(&mut self, movement: &RobotMovement) {
        let (dx, dy) = movement.delta();

        let mut boxes_to_move = HashSet::new();
        let mut tail_queue = VecDeque::new();
        tail_queue.push_back(self.robot.position.clone().add(dx, dy));

        while let Some(tail) = tail_queue.pop_front() {
            if boxes_to_move.contains(&tail) {
                continue;
            }

            match self.grid.get(&tail) {
                Some(Tile::Wall) => {
                    // The head box cannot be pushed
                    return;
                }
                Some(Tile::SmallBox) => {
                    boxes_to_move.insert(tail.clone());
                    tail_queue.push_back(tail.add(dx, dy));
                }
                Some(Tile::BigBoxFirstHalf) => {
                    boxes_to_move.insert(tail.clone());

                    // Add the right part of the box to the queue
                    tail_queue.push_back(tail.add(1, 0));
                    tail_queue.push_back(tail.add(dx, dy));
                }
                Some(Tile::BigBoxSecondHalf) => {
                    boxes_to_move.insert(tail.clone());

                    // Add the right part of the box to the queue
                    tail_queue.push_back(tail.add(-1, 0));
                    tail_queue.push_back(tail.add(dx, dy));
                }
                None => {}
            }
        }

        let mut sorted_boxes_to_move: Vec<_> = boxes_to_move.iter().collect();
        match movement {
            RobotMovement::Up => sorted_boxes_to_move.sort_by_key(|position| position.y),
            RobotMovement::Down => sorted_boxes_to_move.sort_by_key(|position| Reverse(position.y)),
            RobotMovement::Left => sorted_boxes_to_move.sort_by_key(|position| position.x),
            RobotMovement::Right => {
                sorted_boxes_to_move.sort_by_key(|position| Reverse(position.x))
            }
        }

        for box_to_move in sorted_boxes_to_move {
            let tile = self
                .grid
                .remove(box_to_move)
                .expect("Tile should be in gried");
            self.grid.insert(box_to_move.add(dx, dy), tile);
        }
        self.robot.position = self.robot.position.add(dx, dy);
    }

    pub fn box_gps_coordinate_sum(&self) -> usize {
        self.grid
            .iter()
            .filter(|(_, &tile)| tile == Tile::SmallBox || tile == Tile::BigBoxFirstHalf)
            .map(|(position, _)| position.gps_coordinate())
            .sum()
    }

    fn grid_max_position(&self) -> (usize, usize) {
        let max_x = self.grid.keys().map(|position| position.x).max().unwrap();
        let max_y = self.grid.keys().map(|position| position.y).max().unwrap();

        (max_x, max_y)
    }
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let (grid_str, robot_movements) = value.split_once("\n\n").unwrap();

        let mut robot_position = None;

        let mut grid = HashMap::new();
        for (y, line) in grid_str.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let position = Position::new(x, y);
                if char == '@' {
                    robot_position = Some(position);
                    continue;
                }

                if let Some(tile) = Tile::from_char(char) {
                    grid.insert(position, tile);
                }
            }
        }

        let robot_movements = robot_movements
            .lines()
            .flat_map(|movements| movements.chars().map(RobotMovement::from))
            .collect();

        Self {
            grid,
            robot: Robot::new(robot_position.unwrap(), robot_movements),
        }
    }
}
