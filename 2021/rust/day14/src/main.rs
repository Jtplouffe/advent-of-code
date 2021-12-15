use crate::polymerizater::Polymerizater;

mod polymerizater;

fn main() {
    let input = include_str!("input");
    let mut polymerizater = Polymerizater::from(input);

    polymerizater.run_steps(10);
    println!(
        "Part one: {}",
        polymerizater.diff_most_common_and_least_common_count()
    );

    polymerizater.run_steps(30);
    println!(
        "Part two: {}",
        polymerizater.diff_most_common_and_least_common_count()
    );
}
