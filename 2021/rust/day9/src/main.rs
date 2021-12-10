use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input");

    let height_map = HeightMap::from(input);

    println!("Part one: {}", height_map.risk_level());

    let mut basin_sizes = height_map.basin_sizes();
    basin_sizes.sort_by(|a, b| b.cmp(a));

    println!("Part two: {}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

#[derive(Debug)]
struct HeightMap {
    heights: Vec<u8>,
    width: usize,
}

impl HeightMap {
    fn risk_level(&self) -> u32 {
        self.heights.iter().enumerate().filter_map(|(index, height)| {
            if self.is_low_point(index) {
                Some((height + 1) as u32)
            } else {
                None
            }
        }).sum()
    }

    fn is_low_point(&self, index: usize) -> bool {
        let current = self.heights.get(index).unwrap();

        let top = self.heights.get(((index as isize) - self.width as isize) as usize).unwrap_or(&9);
        let bottom = self.heights.get(index + self.width).unwrap_or(&9);

        let left = if index % self.width > 0 {
            self.heights.get(index - 1)
        } else {
            None
        }.unwrap_or(&9);

        let right = if index % self.width < self.width - 1 {
            self.heights.get(index + 1)
        } else {
            None
        }.unwrap_or(&9);

        current < top && current < bottom && current < left && current < right
    }

    fn basin_sizes(&self) -> Vec<usize> {
        (0..self.heights.len()).filter_map(|index| {
            if self.is_low_point(index) {
                Some(self.basin_size(index))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }

    fn basin_size(&self, index: usize) -> usize {
        let mut basin = HashSet::new();
        self.get_basin_indexes_for_index(index, &mut basin);
        basin.len()
    }

    fn get_basin_indexes_for_index(&self, index: usize, basin: &mut HashSet<usize>) {
        if self.heights[index] == 9 || basin.contains(&index) {
            return;
        }

        basin.insert(index);

        if index % self.width > 0 {
            self.get_basin_indexes_for_index(index - 1, basin);
        }
        if index % self.width < self.width - 1 {
            self.get_basin_indexes_for_index(index + 1, basin);
        }
        if index >= self.width {
            self.get_basin_indexes_for_index(index - self.width, basin);
        }
        if index < self.heights.len() - self.width {
            self.get_basin_indexes_for_index(index + self.width, basin);
        }
    }
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let heights: Vec<_> = s.split("").filter_map(|i| u8::from_str(i).ok()).collect();
        let width = s.lines().nth(0).unwrap().len();

        Self { heights, width }
    }
}
