use std::collections::{HashMap, HashSet, VecDeque};

use crate::cube::Cube;

pub struct Grid {
    cubes: Vec<Cube>,
}

impl Grid {
    pub fn apply_gravity(&mut self) {
        self.sort_cubes();

        for cube_index in 0..self.cubes.len() {
            loop {
                let fallen_cube = self.cubes[cube_index].copy_with_offset(0, -1, 0);
                if fallen_cube.y1.min(fallen_cube.y2) == 0 {
                    break;
                }

                let mut can_move_down = true;
                for other_cube_index in 0..self.cubes.len() {
                    if other_cube_index == cube_index {
                        continue;
                    }

                    if self.cubes[other_cube_index].overlaps(&fallen_cube) {
                        can_move_down = false;
                        break;
                    }
                }

                if !can_move_down {
                    break;
                }

                self.cubes[cube_index] = fallen_cube;
            }
        }
    }

    pub fn first_desintegratable_brick_count(&self) -> usize {
        let cube_indexes_direct_dependencies = self.cube_indexes_direct_dependencies();
        self.cube_indexes_direct_dependents()
            .values()
            .filter(|dependents| {
                dependents.iter().all(|dependent| {
                    cube_indexes_direct_dependencies
                        .get(dependent)
                        .unwrap()
                        .len()
                        > 1
                })
            })
            .count()
    }

    pub fn every_brick_fall_chain_reaction_count(&self) -> usize {
        let cube_indexes_direct_dependencies = self.cube_indexes_direct_dependencies();
        let cube_indexes_direct_dependents = self.cube_indexes_direct_dependents();

        let mut count = 0;

        for (&cube_index, direct_dependents) in &cube_indexes_direct_dependents {
            let mut desintegrated_cubes = HashSet::from([cube_index]);

            let mut queue = VecDeque::new();
            for &next_cube_index in direct_dependents {
                queue.push_back(next_cube_index);
            }

            while let Some(cube_index) = queue.pop_front() {
                let direct_dependencies =
                    cube_indexes_direct_dependencies.get(&cube_index).unwrap();

                if direct_dependencies
                    .iter()
                    .any(|direct_dependency| !desintegrated_cubes.contains(direct_dependency))
                {
                    continue;
                }

                desintegrated_cubes.insert(cube_index);

                for &next_cube_index in cube_indexes_direct_dependents.get(&cube_index).unwrap() {
                    queue.push_back(next_cube_index);
                }
            }

            count += desintegrated_cubes.len() - 1
        }

        count
    }

    fn sort_cubes(&mut self) {
        self.cubes.sort_unstable_by_key(|cube| cube.y1.min(cube.y2));
    }

    fn cube_indexes_direct_dependencies(&self) -> HashMap<usize, Vec<usize>> {
        let mut hash_map = HashMap::new();

        for (cube_index, cube) in self.cubes.iter().enumerate() {
            let mut dependencies = Vec::new();

            for (other_cube_index, other_cube) in self.cubes.iter().enumerate() {
                if other_cube_index == cube_index || !cube.is_supported_by(other_cube) {
                    continue;
                }

                dependencies.push(other_cube_index);
            }

            hash_map.insert(cube_index, dependencies);
        }

        hash_map
    }

    fn cube_indexes_direct_dependents(&self) -> HashMap<usize, Vec<usize>> {
        let mut hash_map = HashMap::new();

        for (cube_index, cube) in self.cubes.iter().enumerate() {
            let mut dependants = Vec::new();

            for (other_cube_index, other_cube) in self.cubes.iter().enumerate() {
                if other_cube_index == cube_index || !other_cube.is_supported_by(cube) {
                    continue;
                }

                dependants.push(other_cube_index);
            }

            hash_map.insert(cube_index, dependants);
        }

        hash_map
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let cubes = value.lines().map(Cube::from).collect();

        Self { cubes }
    }
}
