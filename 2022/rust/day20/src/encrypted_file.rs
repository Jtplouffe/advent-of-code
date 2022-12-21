pub struct EncryptedFile {
    buffer: Vec<(usize, i64)>,
}

impl EncryptedFile {
    pub fn apply_key(&mut self, key: i64) {
        for i in 0..self.buffer.len() {
            self.buffer[i].1 *= key;
        }
    }

    pub fn decrypt(&mut self, rounds: usize) {
        for _ in 0..rounds {
            for position in 0..self.buffer.len() {
                let index = self
                    .buffer
                    .iter()
                    .enumerate()
                    .find_map(|(index, &value)| (value.0 == position).then_some(index))
                    .unwrap();
                let current_value = self.buffer.remove(index);

                let mut new_position =
                    (index as i64 + current_value.1).rem_euclid(self.buffer.len() as i64) as usize;
                if current_value.1 < 0 && new_position == 0 {
                    new_position = self.buffer.len();
                }

                self.buffer.insert(new_position, current_value);
            }
        }
    }

    pub fn grove_coordinates(&self) -> i64 {
        let zero_index = self
            .buffer
            .iter()
            .enumerate()
            .find_map(|(index, &value)| (value.1 == 0).then_some(index))
            .unwrap();

        let mut sum = 0;
        for i in &[1000 + zero_index, 2000 + zero_index, 3000 + zero_index] {
            sum += self.buffer[i % self.buffer.len()].1
        }

        sum
    }
}

impl From<&str> for EncryptedFile {
    fn from(s: &str) -> Self {
        let buffer = s
            .lines()
            .enumerate()
            .map(|(index, value)| (index, value.parse().unwrap()))
            .collect();

        Self { buffer }
    }
}
