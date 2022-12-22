use riddle::Riddle;

mod monkey;
mod riddle;

fn main() {
    let input = include_str!("./input");

    let mut riddle = Riddle::from(input);
    println!("Part 1: {}", riddle.solve_part_1());
    println!("Part 2: {}", riddle.solve_part_2());
}
