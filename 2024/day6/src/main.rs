use map::Map;

mod map;
mod position;

fn main() {
    let input = include_str!("./input").trim_end();

    let map = Map::from(input);

    let guard_positions = map.guard_positions();
    println!("Part 1: {}", guard_positions.len());

    let possible_looping_obstactes = map.possible_looping_obstructions();
    println!("Part 2: {}", possible_looping_obstactes.len());
}
