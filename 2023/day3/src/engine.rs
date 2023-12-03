use std::{collections::HashSet, ops::Add};

pub struct Engine {
    schematic: Vec<char>,
    schematic_width: usize,
}

impl Engine {
    pub fn parts_sum(&self) -> u32 {
        let mut sum = 0;
        let mut handled_number_starting_indexes = HashSet::new();

        for (symbol_index, _) in self.iter_symbols() {
            for (digit_index, _) in self.iter_digits_around_index(symbol_index) {
                let (number_starting_index, number) = self.full_number_at_index(digit_index);

                if handled_number_starting_indexes.contains(&number_starting_index) {
                    continue;
                }

                sum += number;
                handled_number_starting_indexes.insert(number_starting_index);
            }
        }

        sum
    }

    pub fn gear_ratios_sum(&self) -> u32 {
        let mut sum = 0;

        for possible_engine_index in self.iter_symbol('*') {
            let numbers_around = self.full_numbers_around_index(possible_engine_index);
            if numbers_around.len() != 2 {
                continue;
            }

            sum += numbers_around
                .iter()
                .map(|(_, number)| *number)
                .product::<u32>();
        }

        sum
    }

    fn index_to_position(&self, index: usize) -> (usize, usize) {
        (index % self.schematic_width, index / self.schematic_width)
    }

    fn position_to_index(&self, x: usize, y: usize) -> usize {
        y * self.schematic_width + x
    }

    fn iter_symbols(&self) -> impl Iterator<Item = (usize, char)> + '_ {
        self.schematic
            .iter()
            .enumerate()
            .filter_map(|(index, &char)| {
                if char != '.' && !char.is_ascii_digit() {
                    Some((index, char))
                } else {
                    None
                }
            })
    }

    fn iter_symbol(&self, symbol: char) -> impl Iterator<Item = usize> + '_ {
        self.iter_symbols()
            .filter_map(move |(index, s)| if s == symbol { Some(index) } else { None })
    }

    fn full_numbers_around_index(&self, index: usize) -> Vec<(usize, u32)> {
        let mut numbers_with_starting_index = Vec::<(usize, u32)>::new();

        for (digit_index, _) in self.iter_digits_around_index(index) {
            let (number_starting_index, number) = self.full_number_at_index(digit_index);
            if numbers_with_starting_index
                .iter()
                .any(|(index, _)| *index == number_starting_index)
            {
                continue;
            }

            numbers_with_starting_index.push((number_starting_index, number));
        }

        numbers_with_starting_index
    }

    fn full_number_at_index(&self, index: usize) -> (usize, u32) {
        let row_start_index = (index / self.schematic_width) * self.schematic_width;
        let row_end_index = row_start_index + self.schematic_width - 1;
        let row = &self.schematic[row_start_index..=row_end_index];

        let index_in_row = index - row_start_index;

        let mut number_starting_index = None;
        let mut number_str = String::new();
        for (char_index, char) in row.iter().enumerate() {
            match char.is_ascii_digit() {
                true => {
                    if number_starting_index.is_none() {
                        number_starting_index = Some(char_index);
                    }

                    number_str.push(*char);
                }
                false => {
                    if char_index > index_in_row {
                        break;
                    }

                    number_starting_index = None;
                    number_str.clear();
                }
            }
        }

        (
            number_starting_index.unwrap() + row_start_index,
            number_str.parse().unwrap(),
        )
    }

    fn iter_digits_around_index(&self, index: usize) -> impl Iterator<Item = (usize, u32)> {
        self.iter_around_index(index)
            .filter_map(|(index, char)| char.to_digit(10).map(|digit| (index, digit)))
    }

    fn iter_around_index(&self, index: usize) -> impl Iterator<Item = (usize, char)> {
        let mut chars = Vec::with_capacity(8);

        let (pos_x, pos_y) = self.index_to_position(index);
        for y in pos_y.saturating_sub(1)
            ..=pos_y
                .add(1)
                .clamp(0, self.schematic.len() / self.schematic_width)
        {
            for x in pos_x.saturating_sub(1)..=pos_x.add(1).clamp(0, self.schematic_width) {
                if x == pos_x && y == pos_y {
                    continue;
                }

                let index = self.position_to_index(x, y);
                chars.push((index, self.schematic[index]));
            }
        }

        chars.into_iter()
    }
}

impl From<&str> for Engine {
    fn from(value: &str) -> Self {
        let schematic: Vec<_> = value.lines().flat_map(|line| line.chars()).collect();
        let schematic_width = value.chars().take_while(|&char| char != '\n').count();

        Self {
            schematic,
            schematic_width,
        }
    }
}
