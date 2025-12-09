use crate::electrical_grid::ElectricalGrid;

mod electrical_grid;
mod position;

fn main() {
    let input = include_str!("./input").trim();
    let electrical_grid = ElectricalGrid::from(input);

    println!(
        "Part 1: {}",
        electrical_grid.three_largest_circuit_size_product(1000)
    );

    println!(
        "Part 2: {}",
        electrical_grid.single_circuit_last_connection_x_product()
    );
}
