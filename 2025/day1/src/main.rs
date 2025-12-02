use crate::{dial::Dial, rotation::Rotation};

mod dial;
mod rotation;

fn main() {
    let input = include_str!("./input").trim();

    let rotations: Vec<_> = input.lines().map(Rotation::from).collect();
    let dial = Dial::new(rotations);

    let zero_occurrences_at_end = dial.count_zero_occurrences_at_end();
    println!("Part 1: {zero_occurrences_at_end}");

    let zero_occurernces_anytime = dial.count_zero_occurrences_anytime();
    println!("Part 2: {zero_occurernces_anytime}");
}
