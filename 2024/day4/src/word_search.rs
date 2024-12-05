type Position = (isize, isize);

const DIRECTION_DELTAS: &[Position] = &[
    (-1, -1), // Top left
    (0, -1),  // Top
    (1, -1),  // Top right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Bottom left
    (0, 1),   // Bottom
    (1, 1),   // Bottom right
];

const XMAS: &str = "XMAS";

pub struct WordSearch {
    grid: Vec<Vec<char>>,
}

impl WordSearch {
    pub fn count_xmas_occurrence(&self) -> usize {
        let mut count = 0;

        let first_char = XMAS.chars().nth(0).unwrap();

        for (start_y, line) in self.grid.iter().enumerate() {
            for (start_x, &char) in line.iter().enumerate() {
                if char != first_char {
                    continue;
                }

                'directions: for (delta_x, delta_y) in DIRECTION_DELTAS {
                    for (index, char) in XMAS.chars().enumerate() {
                        let x = start_x as isize + (delta_x * index as isize);
                        let y = start_y as isize + (delta_y * index as isize);
                        if x < 0 || y < 0 {
                            continue 'directions;
                        }

                        let (x, y) = (x as usize, y as usize);

                        if y >= self.grid.len() || x >= self.grid[y].len() {
                            continue 'directions;
                        }

                        if self.grid[y][x] != char {
                            continue 'directions;
                        }
                    }

                    count += 1;
                }
            }
        }

        count
    }

    pub fn count_mas_occurrence(&self) -> usize {
        let mut count = 0;

        // First and last lines will never have the middle part of 'MAS' because of bounds
        for (middle_y, line) in self.grid[1..self.grid.len() - 1].iter().enumerate() {
            // We need to increase by 1 since we skipped the first line
            let middle_y = middle_y + 1;

            // First and last chars will never have the middle part of 'MAS' because of bounds
            for (middle_x, &char) in line[1..line.len() - 1].iter().enumerate() {
                // We need to increase by 1 since we skipped the first char
                let middle_x = middle_x + 1;

                if char != 'A' {
                    continue;
                }

                let first_start_char = self.grid[middle_y - 1][middle_x - 1];
                let first_end_char = self.grid[middle_y + 1][middle_x + 1];
                if !(first_start_char == 'M' && first_end_char == 'S')
                    && !(first_start_char == 'S' && first_end_char == 'M')
                {
                    continue;
                }

                let second_start_char = self.grid[middle_y - 1][middle_x + 1];
                let second_end_char = self.grid[middle_y + 1][middle_x - 1];
                if !(second_start_char == 'M' && second_end_char == 'S')
                    && !(second_start_char == 'S' && second_end_char == 'M')
                {
                    continue;
                }

                count += 1;
            }
        }

        count
    }
}

impl From<&str> for WordSearch {
    fn from(value: &str) -> Self {
        let grid = value.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }
}
