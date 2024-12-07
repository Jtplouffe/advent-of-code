use equation::Equation;
use operators::Operators;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod equation;
mod operators;

const FIRST_OPERATORS: &[Operators] = &[Operators::Addition, Operators::Multiplication];
const SECOND_OPERATORS: &[Operators] = &[
    Operators::Addition,
    Operators::Multiplication,
    Operators::Concatenation,
];

fn main() {
    let input = include_str!("./input").trim_end();

    let equations: Vec<_> = input.lines().map(Equation::from).collect();

    let first_total_calibration_result: u64 = equations
        .par_iter()
        .filter(|equation| equation.could_be_true(FIRST_OPERATORS))
        .map(|equation| equation.total)
        .sum();
    println!("Part 1: {first_total_calibration_result}");

    let second_total_calibration_result: u64 = equations
        .par_iter()
        .filter(|equation| equation.could_be_true(SECOND_OPERATORS))
        .map(|equation| equation.total)
        .sum();
    println!("Part 1: {second_total_calibration_result}");
}
