use map::Map;

mod direction;
mod lcm;
mod map;

fn main() {
    let input = include_str!("./input");

    let map = Map::from(input);

    let steps = map.steps_to_end();
    println!("Part 1: {steps}");

    let multi_nodes_steps = map.multi_node_steps_to_end_fast();
    println!("Part 2: {multi_nodes_steps}");
}
