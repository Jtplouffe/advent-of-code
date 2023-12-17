use grid::Grid;

mod direction;
mod grid;

fn main() {
    let input = include_str!("./input");

    let grid = Grid::from(input);

    let min_heat_loss = grid.min_heat_loss(1, 3);
    println!("Part 1: {min_heat_loss}");

    let min_heat_loss = grid.min_heat_loss(4, 10);
    println!("Part 2: {min_heat_loss}");
}
