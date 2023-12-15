use crate::{hasher::hash, lens_boxes::LensBoxes, step::Step};

mod hasher;
mod lens;
mod lens_boxes;
mod step;

fn main() {
    let input = include_str!("./input");

    let checksum: u32 = input.trim().split(',').map(hash).sum();
    println!("Part 1: {checksum}");

    let steps: Vec<_> = input.trim().split(',').map(Step::from).collect();
    let mut lens_boxes = LensBoxes::new();
    lens_boxes.execute_steps(&steps);

    let focusing_power = lens_boxes.focusing_power();
    println!("Part 1: {focusing_power}");
}
