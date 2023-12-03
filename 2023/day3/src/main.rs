use engine::Engine;

mod engine;

fn main() {
    let input = include_str!("./input");

    let engine = Engine::from(input);

    let parts_sum = engine.parts_sum();
    println!("Part 1: {parts_sum}");

    let gear_ratios_sum = engine.gear_ratios_sum();
    println!("Part 2: {gear_ratios_sum}");
}
