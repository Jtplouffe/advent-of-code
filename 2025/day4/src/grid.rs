use std::collections::HashSet;

type Position = (isize, isize);
const RELATIVE_POSITIONS: [Position; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct Grid {
    positions: HashSet<Position>,
}

impl Grid {
    pub fn accessible_roll_count(&self, with_removing: bool) -> usize {
        let mut positions = self.positions.clone();
        let mut accessed_positions = HashSet::new();

        loop {
            for position in &positions {
                let rolls_around = Self::count_rolls_around(&positions, position);
                if rolls_around < 4 {
                    accessed_positions.insert(*position);
                }
            }

            for accessed_position in &accessed_positions {
                positions.remove(accessed_position);
            }

            if !with_removing || accessed_positions.is_empty() {
                break;
            }

            accessed_positions.clear();
        }

        self.positions.len() - positions.len()
    }

    fn count_rolls_around(positions: &HashSet<Position>, (x, y): &Position) -> usize {
        let mut count = 0;
        for (rx, ry) in RELATIVE_POSITIONS {
            if positions.contains(&(x + rx, y + ry)) {
                count += 1;
            }
        }

        count
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut positions = HashSet::new();

        let lines: Vec<_> = value.lines().collect();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '@' {
                    positions.insert((x as isize, y as isize));
                }
            }
        }

        Self { positions }
    }
}
