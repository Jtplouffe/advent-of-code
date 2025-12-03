pub struct BatteryBank {
    joltages: Vec<u64>,
}

impl BatteryBank {
    pub fn max_joltage(&self, battery_count: usize) -> u64 {
        let mut digit_indexes = Vec::<usize>::with_capacity(battery_count);
        let mut max_joltage = 0;

        for battery_index in 0..battery_count {
            let min_index = digit_indexes
                .last()
                .map(|index| index + 1)
                .unwrap_or_else(|| 0);
            let max_index = self.joltages.len() - battery_count + battery_index;

            let available_digits = &self.joltages[min_index..=max_index];

            let digit = available_digits.iter().max().expect("No joltages");
            let digit_index = available_digits
                .iter()
                .enumerate()
                .find_map(|(index, joltage)| {
                    if joltage == digit {
                        Some(index + min_index)
                    } else {
                        None
                    }
                })
                .unwrap();

            digit_indexes.push(digit_index);

            max_joltage += digit * 10u64.pow((battery_count - battery_index - 1) as u32);
        }

        max_joltage
    }
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let joltages = value
            .chars()
            .map(|c| c.to_digit(10).expect("Invaild number") as _)
            .collect();

        Self { joltages }
    }
}
