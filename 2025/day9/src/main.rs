use crate::floor::Floor;

mod floor;
mod position;

fn main() {
    let input = include_str!("./input").trim();
    let floor = Floor::from(input);

    println!("Part 1: {}", floor.largest_rectangle());
    // println!("Part 2: {}", floor.lagset_rectangle_within_area());
}
