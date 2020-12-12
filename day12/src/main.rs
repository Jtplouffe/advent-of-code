use crate::boat::Boat;
use crate::action::Action;

mod boat;
mod direction;
mod action;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let actions: Vec<Action> = input.lines().map(|line| line.into()).collect();

    let mut boat = Boat::new();
    boat.navigate_part_one(&actions);

    println!("Part 1: {}", boat.manhattan_distance());

    boat.reset();
    boat.navigate_part_two(&actions);

    println!("Part 2: {}", boat.manhattan_distance());
}
