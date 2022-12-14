use std::collections::HashMap;

use crate::point::Point;

#[derive(Copy, Clone)]
enum CaveMaterial {
    Rock,
    Sand,
}

#[derive(Clone)]
pub struct Cave {
    grid: HashMap<Point, CaveMaterial>,
}

impl Cave {
    pub fn sand_count(&self) -> usize {
        self.grid
            .values()
            .filter(|&material| match material {
                CaveMaterial::Sand => true,
                _ => false,
            })
            .count()
    }

    pub fn produce_sand(&mut self, with_floor: bool) {
        while !self.generate_and_fall_sand(with_floor) {}
    }

    fn generate_and_fall_sand(&mut self, with_floor: bool) -> bool {
        let mut sand_position = Point::new(500, 0);
        if with_floor && self.grid.contains_key(&sand_position) {
            return true;
        }

        let lowest_wall = self
            .grid
            .iter()
            .filter_map(|(point, &material)| match material {
                CaveMaterial::Rock => Some(point),
                _ => None,
            })
            .reduce(|lowest, current| {
                if current.y > lowest.y {
                    current
                } else {
                    lowest
                }
            })
            .unwrap();

        loop {
            if with_floor {
                if sand_position.y == lowest_wall.y + 1 {
                    break;
                }
            } else {
                if sand_position.y >= lowest_wall.y {
                    return true;
                }
            }

            let down = sand_position.add(0, 1);
            if !self.grid.contains_key(&down) {
                sand_position = down;
                continue;
            }

            let down_left = sand_position.add(-1, 1);
            if !self.grid.contains_key(&down_left) {
                sand_position = down_left;
                continue;
            }

            let down_right = sand_position.add(1, 1);
            if !self.grid.contains_key(&down_right) {
                sand_position = down_right;
                continue;
            }

            break;
        }

        self.grid.insert(sand_position, CaveMaterial::Sand);

        false
    }
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        let walls: Vec<Vec<_>> = s
            .lines()
            .map(|wall| wall.split(" -> ").map(Point::from).collect())
            .collect();

        let grid: HashMap<_, _> = walls
            .iter()
            .flat_map(|wall| {
                wall.windows(2)
                    .flat_map(|points| points[0].range_to_inclusive(&points[1]))
            })
            .map(|point| (point, CaveMaterial::Rock))
            .collect();

        Self { grid }
    }
}
