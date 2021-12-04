use std::str::FromStr;

#[derive(Debug)]
pub struct BingoBoard {
    numbers: Vec<u32>,
    marks: Vec<bool>,
    pub has_won: bool,
}

impl BingoBoard {
    fn size(&self) -> usize {
        (self.numbers.len() as f32).sqrt() as usize
    }

    pub fn picked_number(&mut self, number: u32) {
        let index = self.numbers.iter().position(|n| *n == number);
        if let Some(index) = index {
            self.marks[index] = true;
            self.calculate_has_won();
        }
    }

    fn calculate_has_won(&mut self) {
        let size = self.size();

        for i in 0..size {
            for j in 0..size {
                if !self.marks[i + j * size] {
                    break;
                }

                if j == size - 1 {
                    self.has_won = true;
                    return;
                }
            }

            for j in 0..size {
                if !self.marks[i * size + j] {
                    break;
                }

                if j == size - 1 {
                    self.has_won = true;
                    return;
                }
            }
        }
    }

    pub fn unmarked_numbers_sum(&self) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .filter_map(
                |(index, num)| {
                    if !self.marks[index] {
                        Some(num)
                    } else {
                        None
                    }
                },
            )
            .sum()
    }

    pub fn reset(&mut self) {
        self.has_won = false;
        self.marks = vec![false; self.numbers.len()];
    }
}

impl From<&str> for BingoBoard {
    fn from(s: &str) -> Self {
        let numbers: Vec<_> = s
            .replace("  ", " ")
            .replace("\n", " ")
            .split(" ")
            .filter_map(|n| u32::from_str(n).ok())
            .collect();

        let count = numbers.len();

        Self {
            numbers,
            marks: vec![false; count],
            has_won: false,
        }
    }
}
