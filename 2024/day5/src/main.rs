use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use safety_protocol::SafetyProtocol;
use update::Update;

mod safety_protocol;
mod update;

fn main() {
    let input = include_str!("./input").trim_end();

    let (raw_rules, raw_updates) = input.split_once("\n\n").unwrap();

    let safety_protocol = SafetyProtocol::new(raw_rules);

    let updates: Vec<_> = raw_updates.lines().map(Update::from).collect();

    let part_1: usize = updates
        .iter()
        .filter(|update| safety_protocol.is_update_safe(update))
        .map(|update| update.middle_page())
        .sum();
    println!("Part 1: {part_1}");

    let part_2: usize = updates
        .par_iter()
        .filter(|update| !safety_protocol.is_update_safe(update))
        .map(|unsafe_update| safety_protocol.fix_update(unsafe_update))
        .map(|update| update.middle_page())
        .sum();
    println!("Part 2: {part_2}");
}
