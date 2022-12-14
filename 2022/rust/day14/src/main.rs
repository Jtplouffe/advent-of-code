use crate::cave::Cave;

mod cave;
mod point;

fn main() {
    let input = include_str!("./input");

    let mut cave_1 = Cave::from(input);
    let mut cave_2 = cave_1.clone();

    cave_1.produce_sand(false);
    println!("Part 1: {}", cave_1.sand_count());

    cave_2.produce_sand(true);
    println!("Part 2: {}", cave_2.sand_count());
}
