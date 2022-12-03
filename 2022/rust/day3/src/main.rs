extern crate core;

use itertools::Itertools;

use crate::group::Group;
use crate::rucksack::Rucksack;

mod group;
mod rucksack;

fn main() {
    let input = include_str!("./input");

    let rucksacks = input.lines().map(Rucksack::from).collect::<Vec<_>>();

    let error_items_priority_sum = rucksacks
        .iter()
        .map(|rucksack| rucksack.error_item().priority())
        .sum::<usize>();
    println!("Part 1: {error_items_priority_sum}");

    let groups = rucksacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunks| Group::new(chunks.collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    let groups_badge_priority_sum = groups.iter().map(|group| group.badge_item().priority()).sum::<usize>();
    println!("Part 2: {groups_badge_priority_sum}");
}
