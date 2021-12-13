use crate::cave::Caves;

mod cave;

fn main() {
    let input = include_str!("input");

    let caves = Caves::from(input);
    println!("Part one: {}", caves.path_to_visit_count_part_1());
    println!("Part two: {}", caves.path_to_visit_count_part_2());
}
