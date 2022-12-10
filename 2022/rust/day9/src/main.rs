use crate::rope_grid::RopeGrid;
use crate::step::Step;

mod rope_grid;
mod step;

fn main() {
    let input = include_str!("./input");

    let steps = input.lines().map(Step::from).collect::<Vec<_>>();

    let mut one_knot_rope_grid = RopeGrid::new(1);
    let tail_position_count = one_knot_rope_grid.navigate(&steps);
    println!("Part 1: {tail_position_count}");

    let mut ten_knot_rope_grid = RopeGrid::new(9);
    let tail_position_count = ten_knot_rope_grid.navigate(&steps);
    println!("Part 2: {tail_position_count}");
}
