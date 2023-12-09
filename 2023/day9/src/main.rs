use oasis_report::OasisReport;

mod history;
mod oasis_report;

fn main() {
    let input = include_str!("./input");

    let oasis_report = OasisReport::from(input);

    let extrapolated_next_values_sum = oasis_report.extrapolated_next_values_sum();
    println!("Part 1: {extrapolated_next_values_sum}");

    let extrapolated_previous_values_sum = oasis_report.extrapolated_previous_values_sum();
    println!("Part 2: {extrapolated_previous_values_sum}");
}
