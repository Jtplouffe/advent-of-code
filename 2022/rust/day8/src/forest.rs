use std::collections::HashSet;

pub struct Forest {
    trees: Vec<Vec<isize>>,
}

impl Forest {
    pub fn visible_trees_count(&self) -> usize {
        let mut highest_tree_positions = HashSet::with_capacity(self.trees.len() * 2 + self.trees[0].len() * 2);

        // Left to right
        for y in 0..self.trees.len() {
            let mut current_highest = -1;

            for x in 0..self.trees[0].len() {
                if self.trees[y][x] > current_highest {
                    highest_tree_positions.insert(cantor_pair(x, y));
                    current_highest = self.trees[y][x];
                }

                if current_highest == 9 {
                    break;
                }
            }
        }

        // Top to bottom
        for x in 0..self.trees[0].len() {
            let mut current_highest = -1;

            for y in 0..self.trees.len() {
                if self.trees[y][x] > current_highest {
                    highest_tree_positions.insert(cantor_pair(x, y));
                    current_highest = self.trees[y][x];
                }

                if current_highest == 9 {
                    break;
                }
            }
        }

        // Right to left
        for y in 0..self.trees.len() {
            let mut current_highest = -1;

            for x in (0..self.trees[0].len()).rev() {
                if self.trees[y][x] > current_highest {
                    highest_tree_positions.insert(cantor_pair(x, y));
                    current_highest = self.trees[y][x];
                }

                if current_highest == 9 {
                    break;
                }
            }
        }

        // Bottom to top
        for x in 0..self.trees[0].len() {
            let mut current_highest = -1;

            for y in (0..self.trees.len()).rev() {
                if self.trees[y][x] > current_highest {
                    highest_tree_positions.insert(cantor_pair(x, y));
                    current_highest = self.trees[y][x];
                }

                if current_highest == 9 {
                    break;
                }
            }
        }

        highest_tree_positions.len()
    }

    pub fn highest_scenic_score(&self) -> isize {
        let mut highest_scenic_score = -1;
        for y in 0..self.trees.len() {
            for x in 0..self.trees[0].len() {
                let scenic_score = self.tree_scenic_score(x, y);
                if scenic_score > highest_scenic_score {
                    highest_scenic_score = scenic_score
                }
            }
        }

        highest_scenic_score
    }

    fn tree_scenic_score(&self, x: usize, y: usize) -> isize {
        let tree_height = self.trees[y][x];

        let mut scenic_score = 1;

        let mut visible_tree_in_direction = 0;
        // Left
        for x_left in (0..x).rev() {
            visible_tree_in_direction += 1;

            if self.trees[y][x_left] >= tree_height {
                break;
            }
        }

        scenic_score *= visible_tree_in_direction;
        visible_tree_in_direction = 0;

        // Top
        for y_top in (0..y).rev() {
            visible_tree_in_direction += 1;

            if self.trees[y_top][x] >= tree_height {
                break;
            }
        }

        scenic_score *= visible_tree_in_direction;
        visible_tree_in_direction = 0;

        // Right
        for x_right in x + 1..self.trees[0].len() {
            visible_tree_in_direction += 1;

            if self.trees[y][x_right] >= tree_height {
                break;
            }
        }

        scenic_score *= visible_tree_in_direction;
        visible_tree_in_direction = 0;

        // Bottom
        for y_bottom in y + 1..self.trees.len() {
            visible_tree_in_direction += 1;

            if self.trees[y_bottom][x] >= tree_height {
                break;
            }
        }

        scenic_score * visible_tree_in_direction
    }
}

impl From<&str> for Forest {
    fn from(s: &str) -> Self {
        let trees = s
            .lines()
            .map(|line| line.chars().map(|tree| tree.to_digit(10).unwrap() as isize).collect())
            .collect();

        Self { trees }
    }
}

fn cantor_pair(n1: usize, n2: usize) -> usize {
    (n1 + n2) * (n1 + n2 + 1) / 2 + n2
}