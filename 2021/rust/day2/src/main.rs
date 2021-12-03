use std::error::Error;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let input = include_str!("input");
    let movements = input.lines().map(Movement::from).collect::<Vec<_>>();

    part_one(&movements);
    part_two(&movements);
}

fn part_one(movements: &Vec<Movement>) {
    let (mut horizontal, mut depth) = (0, 0);
    for movement in movements {
        match movement {
            Movement::Forward(value) => horizontal += value,
            Movement::Down(value) => depth += value,
            Movement::Up(value) => depth -= value,
        }
    }

    println!("Part one: {}", horizontal * depth);
}

fn part_two(movements: &Vec<Movement>) {
    let (mut aim, mut horizontal, mut depth) = (0, 0, 0);
    for movement in movements {
        match movement {
            Movement::Forward(value) => {
                horizontal += value;
                depth += value * aim;
            }
            Movement::Down(value) => aim += value,
            Movement::Up(value) => aim -= value,
        };
    }

    println!("Part two: {}", horizontal * depth);
}

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Movement {
    pub fn get_x(&self) -> i32 {
        match self {
            Movement::Forward(x) => *x,
            _ => 0
        }
    }

    pub fn get_y(&self) -> i32 {
        match self {
            Movement::Down(y) => *y,
            Movement::Up(y) => -*y,
            _ => 0
        }
    }
}

impl From<&str> for Movement {
    fn from(s: &str) -> Self {
        let (direction, raw_distance) = s.split_once(" ").unwrap();
        let distance = i32::from_str(raw_distance).unwrap();
        match direction {
            "forward" => Self::Forward(distance),
            "down" => Self::Down(distance),
            "up" => Self::Up(distance),
            dir => panic!("Unknown direction: {}", dir),
        }
    }
}
