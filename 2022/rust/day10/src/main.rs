use crate::cpu::Cpu;
use crate::instruction::Instruction;

mod cpu;
mod instruction;

fn main() {
    let input = include_str!("./input");
    let instructions = input.lines().map(Instruction::from).collect::<Vec<_>>();

    let mut cpu = Cpu::new();

    for instruction in &instructions {
        cpu.execute_instruction(instruction);
    }

    println!(
        "Part 1: {}",
        cpu.signal_strength_sum_at_cycles(&[20, 60, 100, 140, 180, 220])
    );

    println!("Part 2:\n{}", cpu.generate_image());
}
