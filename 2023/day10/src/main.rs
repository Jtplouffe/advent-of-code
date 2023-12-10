use grid::Grid;

mod direction;
mod grid;
mod pipe;
mod tile;

fn main() {
    let input = include_str!("./input");

    let grid = Grid::from(input);
    let max_pipe_length_from_start = grid.max_length_from_start();
    println!("Part 1: {max_pipe_length_from_start}");
}
