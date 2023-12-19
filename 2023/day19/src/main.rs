use part_ratings::PartRatings;
use system::System as Sys;

mod part_rating_constraints;
mod part_ratings;
mod system;
mod workflow;
mod workflow_destination;
mod workflow_rule;

fn main() {
    let input = include_str!("./input");

    let (system, parts_ratings) = input.split_once("\n\n").unwrap();

    let system = Sys::from(system);
    let parts_ratings: Vec<_> = parts_ratings.lines().map(PartRatings::from).collect();

    let accepted_parts_ratings_sum: u32 = parts_ratings
        .iter()
        .filter_map(|part_ratings| {
            if system.is_part_ratings_accepted(part_ratings) {
                Some(part_ratings.values_sum())
            } else {
                None
            }
        })
        .sum();
    println!("Part 1: {accepted_parts_ratings_sum}");

    let acceptable_distinct_combination_count = system.acceptable_distinct_combination_count();
    println!("Part 2: {acceptable_distinct_combination_count}");
}
