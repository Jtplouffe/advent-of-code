use crate::range::{InvalidIdMode, Range};

mod range;

fn main() {
    let input = include_str!("./input").trim();
    let ranges: Vec<_> = input.split(',').map(Range::from).collect();

    let part_1: usize = ranges
        .iter()
        .map(|range| range.invalid_ids_sum(InvalidIdMode::SingleRepetition))
        .sum();
    println!("Part 1: {part_1}");

    let part_2: usize = ranges
        .iter()
        .map(|range| range.invalid_ids_sum(InvalidIdMode::MultipleRepetition))
        .sum();
    println!("Part 2: {part_2}");
}
