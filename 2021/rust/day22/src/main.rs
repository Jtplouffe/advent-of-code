use crate::engine::Engine;

mod reboot_step;
mod engine;

fn main() {
    let input = include_str!("input");
    let mut engine = Engine::from(input);
    engine.reboot();
    println!("Part one: {}", engine.on_cube_count());
}
