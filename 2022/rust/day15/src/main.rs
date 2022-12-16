use crate::grid::Grid;
use crate::sensor::Sensor;

mod grid;
mod position;
mod sensor;

fn main() {
    let input = include_str!("./input");

    let sensors = input.lines().map(Sensor::from).collect::<Vec<_>>();

    let grid = Grid::new(&sensors);
    println!("Part 1: {}", grid.sensor_coverage_at_y(2000000));
}
