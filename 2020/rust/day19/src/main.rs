#![feature(str_split_once)]

use crate::rule::Rule;
use std::collections::HashMap;

mod rule;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let (raw_rules, data) = input.split_once("\n\n").and_then(|(r, d)| Some((r, d.lines().collect::<Vec<_>>()))).unwrap();
    let mut rules: HashMap<usize, _> = raw_rules.lines().map(Rule::from).map(|r| (r.id, r)).collect();

    let part1 = rules[&0].valid(&data, &rules);
    println!("Part 1: {}", part1);
}
