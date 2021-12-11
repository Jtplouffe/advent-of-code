use crate::octopus_grid::OctopusGrid;

mod octopus;
mod octopus_grid;

fn main() {
    let input = include_str!("input");

    let mut octopus_grid = OctopusGrid::from(input);

    let flash_after_100_steps = octopus_grid.flash_count_after_steps(100);
    println!("Part one: {}", flash_after_100_steps);

    octopus_grid.reset();

    let first_step_with_synchronized_flash = octopus_grid.first_step_with_synchronized_flash();
    println!("Part two: {}", first_step_with_synchronized_flash);
}
