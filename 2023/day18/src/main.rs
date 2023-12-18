use lagoon::Lagoon;

mod dig_plan_line;
mod direction;
mod lagoon;
mod position;

fn main() {
    let input = include_str!("./input");

    let mut lagoon = Lagoon::from(input);

    lagoon.dig();
    
    let capaticy = lagoon.capacity();
    println!("Part 1: {capaticy}");
}
