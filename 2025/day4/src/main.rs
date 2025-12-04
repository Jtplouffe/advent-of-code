use crate::grid::Grid;

mod grid;

fn main() {
    let input = include_str!("./input").trim();

    let grid = Grid::from(input);

    let accisseble_roll_count_without_removing = grid.accessible_roll_count(false);
    println!("Part 1: {accisseble_roll_count_without_removing}");

    let accisseble_roll_count_with_removing = grid.accessible_roll_count(true);
    println!("Part 2: {accisseble_roll_count_with_removing}");
}
