use memory_space::MemorySpace;

mod memory_space;
mod position;

fn main() {
    let input = include_str!("./input").trim_end();

    let memory_space = MemorySpace::new(71, 71, input);

    let distance = memory_space.shortest_distance_at_time(1024);
    println!("Part 1: {distance}");

    let first_byte_to_block_exit = memory_space.first_byte_to_block_exit();
    println!("Part 2: {first_byte_to_block_exit}");
}
