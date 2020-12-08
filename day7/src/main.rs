#![feature(str_split_once)]

use crate::bag_content::BagContent;
use std::collections::HashMap;

mod bag_content;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let bags: HashMap<String, Vec<BagContent>> = input.lines()
        .map(|bag| bag.replace(" bags", "").replace(" bag", "").replace(".", ""))
        .map(|bag| {
            let (color, raw_content) = bag.split_once(" contain ").unwrap();
            let content: Vec<BagContent> = if !raw_content.starts_with("no ") {
                raw_content.split(", ").map(|c| c.parse::<BagContent>().unwrap()).collect()
            } else {
                vec![]
            };
            (color.to_string(), content)
        }).collect();

    let part1 = bags.iter().filter(|(bag_color, bag_content)| {
        bag_color.as_str() != "shiny gold" && bag_content.iter().any(|bag| bag.contains("shiny gold", &bags))
    }).count();

    println!("Part 1: {}", part1);

    let part2: usize = bags.get("shiny gold").unwrap().iter().map(|bag| bag.count_bags_inside(&bags, bag.count as usize)).sum();
    println!("Part 2: {}", part2);
}
