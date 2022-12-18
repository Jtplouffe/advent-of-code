use std::collections::{HashSet, VecDeque};

use crate::position::{Direction, Position};

pub struct Droplet<'a> {
    cubes: HashSet<&'a Position>,
}

impl<'a> Droplet<'a> {
    pub fn new(cubes: &'a [Position]) -> Self {
        Self {
            cubes: HashSet::from_iter(cubes),
        }
    }

    pub fn surface_area(&self) -> usize {
        let mut surface_area = 0;

        for cube in &self.cubes {
            for neighbor in cube.neighbors() {
                if !self.cubes.contains(&neighbor) {
                    surface_area += 1;
                }
            }
        }

        surface_area
    }

    pub fn reachable_surface_area(&self) -> usize {
        let (min_position, max_position) = self.bounds();
        let mut reachable_surface_area = 0;

        let mut visited = HashSet::<(Position, Direction)>::new();
        let mut visitation_queue = VecDeque::<Position>::new();

        visitation_queue.push_front(min_position);
        visited.insert((min_position, Direction::Bottom));

        while let Some(cube) = visitation_queue.pop_front() {
            if self.cubes.contains(&cube) {
                reachable_surface_area += 1;
                continue;
            }

            for (neighbor, connected_from_direction) in cube.neighbors_with_direction() {
                if !visited.contains(&(neighbor, connected_from_direction))
                    && neighbor.is_in_bounds(&min_position, &max_position)
                {
                    visited.insert((neighbor, connected_from_direction));
                    visitation_queue.push_back(neighbor);
                }
            }
        }

        reachable_surface_area
    }

    fn bounds(&self) -> (Position, Position) {
        let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) = (
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
        );

        for cube in &self.cubes {
            if cube.x < min_x {
                min_x = cube.x;
            }

            if cube.x > max_x {
                max_x = cube.x;
            }

            if cube.y < min_y {
                min_y = cube.y;
            }

            if cube.y > max_y {
                max_y = cube.y;
            }

            if cube.z < min_z {
                min_z = cube.z;
            }

            if cube.z > max_z {
                max_z = cube.z;
            }
        }

        (
            Position::new(min_x - 1, min_y - 1, min_z - 1),
            Position::new(max_x + 1, max_y + 1, max_z + 1),
        )
    }
}
