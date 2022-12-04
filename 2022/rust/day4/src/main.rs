use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("./input");

    let pairs = input.lines().map(Pair::from).collect::<Vec<_>>();

    let fully_overlapping_pairs_count = pairs
        .iter()
        .filter(|pair| pair.any_fully_overlaps_other())
        .count();
    println!("Part 1: {fully_overlapping_pairs_count}");

    let partially_overlapping_pairs_count = pairs
        .iter()
        .filter(|pair| pair.any_partially_overlaps_other())
        .count();
    println!("Part 2: {partially_overlapping_pairs_count}");
}

struct Pair {
    first_range: RangeInclusive<usize>,
    second_range: RangeInclusive<usize>,
}

impl Pair {
    fn any_fully_overlaps_other(&self) -> bool {
        (self.first_range.start() >= self.second_range.start()
            && self.first_range.end() <= self.second_range.end())
            || (self.second_range.start() >= self.first_range.start()
                && self.second_range.end() <= self.first_range.end())
    }

    fn any_partially_overlaps_other(&self) -> bool {
        self.first_range.start() <= self.second_range.end()
            && self.second_range.start() <= self.first_range.end()
    }
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let (first_part, second_part) = s.split_once(',').unwrap();

        let (first_part_from, first_part_to) = first_part.split_once('-').unwrap();
        let (second_part_from, second_part_to) = second_part.split_once('-').unwrap();

        Self {
            first_range: first_part_from.parse().unwrap()..=first_part_to.parse().unwrap(),
            second_range: second_part_from.parse().unwrap()..=second_part_to.parse().unwrap(),
        }
    }
}
