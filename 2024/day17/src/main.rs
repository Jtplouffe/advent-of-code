use computer::Computer;

mod computer;
mod instruction;

fn main() {
    let input = include_str!("./input").trim_end();

    let mut computer1 = Computer::from(input);
    let computer2 = computer1.clone();

    let outputs = computer1.execute();
    println!("Part 1: {}", outputs.join(","));

    let register_a = computer2.find_register_a_value_for_prgoram_replication();
    println!("Part 2: {}", register_a);
}
