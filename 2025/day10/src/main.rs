use crate::machine::Machine;

mod indicator_lights;
mod joltages;
mod machine;

fn main() {
    let input = include_str!("./input").trim();

    let machines: Vec<_> = input.lines().map(Machine::from).collect();

    let target_indicator_lights_fewest_button_presses: usize = machines
        .iter()
        .map(Machine::target_indicator_lights_fewest_button_presses)
        .sum();
    println!("Part 1: {target_indicator_lights_fewest_button_presses}");

    // let joltage_requirements_lights_fewest_button_presses: usize = machines
    //     .iter()
    //     .map(Machine::joltage_requirements_fewest_button_presses)
    //     .sum();
    // println!("Part 1: {joltage_requirements_lights_fewest_button_presses}");
}
