use crate::battery_bank::BatteryBank;

mod battery_bank;

fn main() {
    let input = include_str!("./input").trim();

    let battery_banks: Vec<_> = input.lines().map(BatteryBank::from).collect();

    let total_joltage_two_batteries: u64 = battery_banks
        .iter()
        .map(|battery_bank| battery_bank.max_joltage(2))
        .sum();
    println!("Part 1: {total_joltage_two_batteries}");

    let total_joltage_twelve_batteries: u64 = battery_banks
        .iter()
        .map(|battery_bank| battery_bank.max_joltage(12))
        .sum();
    println!("Part 2: {total_joltage_twelve_batteries}");
}
