pub struct Card {
    pub id: usize,
    winning_numbers: Vec<u8>,
    actual_numbers: Vec<u8>,
}

impl Card {
    pub fn matching_number_count(&self) -> usize {
        self.actual_numbers
            .iter()
            .filter(|actual_number| self.winning_numbers.contains(actual_number))
            .count()
    }

    pub fn points(&self) -> u32 {
        self.points_from_matching_number_count(self.matching_number_count())
    }

    fn points_from_matching_number_count(&self, matching_number_count: usize) -> u32 {
        if matching_number_count <= 1 {
            matching_number_count as u32
        } else {
            let mut points = 1;
            for _ in 0..matching_number_count - 1 {
                points *= 2;
            }

            points
        }
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (header, body) = value
            .split_once(": ")
            .expect("expected card header / body delimiter");
        let id = header
            .split(' ')
            .last()
            .expect("expected space")
            .parse()
            .expect("expected number");

        let (winning_numbers, actual_numbers) =
            body.split_once(" | ").expect("expected body delimiter");

        let number_filter_map_fn = |number: &str| {
            if number.is_empty() {
                return None;
            }

            Some(number.parse::<u8>().expect("expected number"))
        };

        let winning_numbers = winning_numbers
            .split(' ')
            .filter_map(number_filter_map_fn)
            .collect();
        let actual_numbers = actual_numbers
            .split(' ')
            .filter_map(number_filter_map_fn)
            .collect();

        Self {
            id,
            winning_numbers,
            actual_numbers,
        }
    }
}
