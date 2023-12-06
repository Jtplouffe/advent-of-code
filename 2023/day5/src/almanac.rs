use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use crate::category_converter::CategoryConverter;

pub struct Almanac<'a> {
    seeds: Vec<usize>,
    category_converters: Vec<CategoryConverter<'a>>,
}

impl<'a> Almanac<'a> {
    pub fn lowest_location(&self) -> usize {
        let mut source_numbers = self.seeds.clone();
        let mut destination_numbers = Vec::with_capacity(source_numbers.len());

        for category_converter in &self.category_converters {
            for &source_number in &source_numbers {
                let destination_number = category_converter
                    .convert(source_number)
                    .unwrap_or(source_number);
                destination_numbers.push(destination_number);
            }

            std::mem::swap(&mut source_numbers, &mut destination_numbers);
            destination_numbers.clear();
        }

        *source_numbers.iter().min().unwrap()
    }

    pub fn lowest_location_from_seed_ranges(&self) -> usize {
        // There is definitively a better way to do this, ig. computing using ranges instead of individual value
        // ... or I can just throw more cpu at it.
        self.seeds
            .par_chunks(2)
            .flat_map(|values| values[0]..values[0] + values[1])
            .map(|seed| {
                let mut source_number = seed;
                let mut destination_number = seed;

                for category_converter in &self.category_converters {
                    destination_number = category_converter
                        .convert(source_number)
                        .unwrap_or(source_number);

                    source_number = destination_number;
                }

                destination_number
            })
            .min()
            .unwrap()
    }
}

impl<'a> From<&'a str> for Almanac<'a> {
    fn from(value: &'a str) -> Self {
        let (seeds, category_converters) = value
            .split_once("\n\n")
            .expect("expected seeds and categoy converters");

        let seeds = seeds
            .split(": ")
            .nth(1)
            .expect("expected `seeds: `")
            .split_whitespace()
            .map(|seed| seed.parse().expect("expected usize"))
            .collect();

        let category_converters = category_converters
            .split("\n\n")
            .map(CategoryConverter::from)
            .collect();

        Self {
            seeds,
            category_converters,
        }
    }
}
