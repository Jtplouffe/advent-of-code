use grid::Grid;

mod cube;
mod grid;

fn main() {
    let input = include_str!("./input");

    let mut grid = Grid::from(input);
    grid.apply_gravity();

    let first_desintegratable_brick_count = grid.first_desintegratable_brick_count();
    println!("Part 1: {first_desintegratable_brick_count}");

    let every_brick_fall_chain_reaction_count = grid.every_brick_fall_chain_reaction_count();
    println!("Part 2: {every_brick_fall_chain_reaction_count}");
}
