use crate::heightmap::Heightmap;

mod heightmap;

fn main() {
    let input = include_str!("./input");

    let heightmap = Heightmap::from(input);

    println!("Part 1: {}", heightmap.fewest_steps_to_destination());
    println!(
        "Part 2: {}",
        heightmap.fewest_steps_to_destination_from_any_start()
    );
}
