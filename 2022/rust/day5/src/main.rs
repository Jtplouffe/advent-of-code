use std::time::Instant;
use crate::action::Action;
use crate::port::Port;

mod action;
mod port;

fn main() {
    let i = Instant::now();
    let input = include_str!("./input");
    let (port, actions) = input.split_once("\n\n").unwrap();

    let actions = actions.lines().map(Action::from).collect::<Vec<_>>();

    let mut port_1 = Port::from(port);
    let mut port_2 = port_1.clone();

    port_1.apply_actions(&actions, false);
    println!("Part 1: {}", port_1.formatted_top_crates());

    port_2.apply_actions(&actions, true);
    println!("Part 2: {}", port_2.formatted_top_crates());
    println!("{:?}", i.elapsed());
}
