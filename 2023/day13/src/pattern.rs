use crate::tile::Tile;

pub struct Pattern {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Pattern {
    pub fn summary(&self, smudge_count: usize) -> usize {
        if let Some(reflection_line_index) = self.vertical_reflection_line(smudge_count) {
            reflection_line_index + 1
        } else if let Some(reflection_line_index) = self.horizontal_reflection_line(smudge_count) {
            (reflection_line_index + 1) * 100
        } else {
            0
        }
    }

    fn vertical_reflection_line(&self, smudge_count: usize) -> Option<usize> {
        let x_to_skip = if smudge_count > 0 {
            self.vertical_reflection_line(0)
        } else {
            None
        };

        'outer: for x in 0..self.width - 1 {
            if let Some(x_to_skip) = x_to_skip {
                if x_to_skip == x {
                    continue;
                }
            }

            let mut used_smudge_count = 0;
            for dx in 0..=x.min(self.width - x - 2) {
                for y in 0..self.height {
                    let left = self.get_tile(x - dx, y);
                    let right = self.get_tile(x + dx + 1, y);

                    if left == right {
                        continue;
                    }

                    if used_smudge_count == smudge_count {
                        continue 'outer;
                    }

                    used_smudge_count += 1;
                }
            }

            return Some(x);
        }

        None
    }

    fn horizontal_reflection_line(&self, smudge_count: usize) -> Option<usize> {
        let y_to_skip = if smudge_count > 0 {
            self.horizontal_reflection_line(0)
        } else {
            None
        };

        'outer: for y in 0..self.height - 1 {
            if let Some(y_to_skip) = y_to_skip {
                if y_to_skip == y {
                    continue;
                }
            }

            let mut used_smudge_count = 0;
            for dy in 0..=y.min(self.height - y - 2) {
                for x in 0..self.width {
                    let left = self.get_tile(x, y - dy);
                    let right = self.get_tile(x, y + dy + 1);

                    if left == right {
                        continue;
                    }

                    if used_smudge_count == smudge_count {
                        continue 'outer;
                    }

                    used_smudge_count += 1;
                }
            }

            return Some(y);
        }

        None
    }

    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.width + x]
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let tiles: Vec<_> = value
            .chars()
            .filter_map(|c| {
                if c.is_whitespace() {
                    None
                } else {
                    Some(Tile::from(c))
                }
            })
            .collect();
        let width = value.lines().next().unwrap().len();
        let height = tiles.len() / width;

        Self {
            tiles,
            width,
            height,
        }
    }
}
