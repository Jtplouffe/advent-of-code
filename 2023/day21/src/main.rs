use map::Map;

mod map;

fn main() {
    let input = include_str!("./input");

    let map = Map::from(input);

    let reached_garden_count = map.reached_garden_count_after_steps(64);
    println!("Part 1: {reached_garden_count}");
}
