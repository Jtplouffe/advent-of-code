use std::collections::HashMap;

type Position = (usize, usize);

pub struct Universe {
    galaxies: Vec<Position>,
}

impl Universe {
    pub fn shortest_pairs_distances_sum(&self, empty_line_value: usize) -> usize {
        let mut pairs_shortest_distances = HashMap::<(Position, Position), usize>::new();

        let empty_columns = self.empty_columns();
        let empty_rows = self.empty_rows();

        for (index, &galaxy) in self.galaxies.iter().enumerate() {
            for (other_index, &other_galaxy) in self.galaxies.iter().enumerate() {
                if index == other_index
                    || pairs_shortest_distances.contains_key(&(galaxy, other_galaxy))
                    || pairs_shortest_distances.contains_key(&(other_galaxy, galaxy))
                {
                    continue;
                }

                let (min_x, max_x) = if galaxy.0 < other_galaxy.0 {
                    (galaxy.0, other_galaxy.0)
                } else {
                    (other_galaxy.0, galaxy.0)
                };
                let empty_column_count = empty_columns
                    .iter()
                    .filter(|&&empty_column| empty_column > min_x && empty_column < max_x)
                    .count();

                let (min_y, max_y) = if galaxy.1 < other_galaxy.1 {
                    (galaxy.1, other_galaxy.1)
                } else {
                    (other_galaxy.1, galaxy.1)
                };
                let empty_row_count = empty_rows
                    .iter()
                    .filter(|&&empty_row| empty_row > min_y && empty_row < max_y)
                    .count();

                let distance_x = (other_galaxy.0 as isize - galaxy.0 as isize).unsigned_abs()
                    + empty_column_count * empty_line_value
                    - empty_column_count;
                let distance_y = (other_galaxy.1 as isize - galaxy.1 as isize).unsigned_abs()
                    + empty_row_count * empty_line_value
                    - empty_row_count;

                let distance = distance_x + distance_y;
                pairs_shortest_distances.insert((galaxy, other_galaxy), distance);
            }
        }

        pairs_shortest_distances.values().sum()
    }

    fn empty_columns(&self) -> Vec<usize> {
        let max_x = self.galaxies.iter().map(|&(x, _)| x).max().unwrap();

        (0..max_x)
            .filter(|x| self.galaxies.iter().all(|(galaxy_x, _)| galaxy_x != x))
            .collect()
    }

    fn empty_rows(&self) -> Vec<usize> {
        let max_y = self.galaxies.iter().map(|&(_, y)| y).max().unwrap();

        (0..max_y)
            .filter(|y| self.galaxies.iter().all(|(_, galaxy_y)| galaxy_y != y))
            .collect()
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let mut galaxies = Vec::new();
        for (y, line) in value.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    galaxies.push((x, y));
                }
            }
        }

        Self { galaxies }
    }
}
