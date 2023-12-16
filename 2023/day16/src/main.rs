use contraption::Contraption;

use crate::direction::Direction;

mod contraption;
mod direction;
mod position;
mod tile;

fn main() {
    let input = include_str!("./input");

    let contraption = Contraption::from(input);

    let top_left_right_energized_tile_count =
        contraption.energized_tile_count((0, 0), Direction::Right);
    println!("Part 1: {top_left_right_energized_tile_count}");

    let max_energized_tile_count = contraption.max_energized_tiles_from_any_starting_point();
    println!("Part 2: {max_energized_tile_count}");
}
