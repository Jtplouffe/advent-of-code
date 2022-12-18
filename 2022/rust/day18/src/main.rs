use std::time::Instant;

use droplet::Droplet;
use position::Position;

mod droplet;
mod position;

fn main() {
    let i = Instant::now();
    let input = include_str!("./input");

    let cubes = input.lines().map(Position::from).collect::<Vec<_>>();
    let droplet = Droplet::new(&cubes);

    println!("Part 1: {}", droplet.surface_area());
    println!("Part 2: {}", droplet.reachable_surface_area());

    println!("{:?}", i.elapsed());
}
