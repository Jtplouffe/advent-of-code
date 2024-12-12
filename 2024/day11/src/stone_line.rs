use std::collections::HashMap;

pub struct StoneLine {
    stones: Vec<u64>,

    stone_count_cache: HashMap<(u64, usize), usize>,
}

impl StoneLine {
    pub fn stone_count_after_blinks(&mut self, blinks: usize) -> usize {
        let mut count = 0;

        for i in 0..self.stones.len() {
            count += self.inner_stone_count_after_blinks(self.stones[i], blinks);
        }

        count
    }

    fn inner_stone_count_after_blinks(&mut self, stone: u64, blinks: usize) -> usize {
        if blinks == 0 {
            return 1;
        }

        let cache_key = (stone, blinks);
        if let Some(&count) = self.stone_count_cache.get(&cache_key) {
            return count;
        }

        let count = if stone == 0 {
            self.inner_stone_count_after_blinks(1, blinks - 1)
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();

                self.inner_stone_count_after_blinks(left, blinks - 1)
                    + self.inner_stone_count_after_blinks(right, blinks - 1)
            } else {
                self.inner_stone_count_after_blinks(stone * 2024, blinks - 1)
            }
        };

        self.stone_count_cache.insert(cache_key, count);
        count
    }
}

impl From<&str> for StoneLine {
    fn from(value: &str) -> Self {
        let stones = value
            .split(" ")
            .map(|stone| stone.parse().unwrap())
            .collect();

        Self {
            stones,
            stone_count_cache: HashMap::new(),
        }
    }
}
