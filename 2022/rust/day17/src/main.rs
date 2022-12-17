use crate::cave::Grid;
use crate::movement::Movement;

mod cave;
mod movement;
mod rock;

fn main() {
    let input = include_str!("./input");

    let movements = input.trim().chars().map(Movement::from).collect::<Vec<_>>();

    let mut grid = Grid::new(&movements);
    grid.generate_rocks(2022);
    println!("Part 1: {}", grid.height());
}
