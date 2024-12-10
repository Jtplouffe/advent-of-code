use map::Map;

mod map;
mod position;

fn main() {
    let input = include_str!("./input").trim_end();

    let map = Map::from(input);

    let antinodes = map.generate_antinodes();
    println!("Part 1: {}", antinodes.len());

    let antinodes_with_harmonic = map.generate_antinodes_with_harmonics();
    println!("Part 2: {}", antinodes_with_harmonic.len());
}
