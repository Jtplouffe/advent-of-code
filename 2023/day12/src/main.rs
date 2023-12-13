use condition_record::ConditionRecord;

mod condition_record;
mod spring_condition;

fn main() {
    let input = include_str!("./input");

    let condition_records: Vec<_> = input.lines().map(ConditionRecord::from).collect();

    let possbile_arrangement_count_sum: usize = condition_records
        .iter()
        .map(|condition_record| condition_record.possible_unfolded_arrangement_count(1))
        .sum();
    println!("Part 1: {possbile_arrangement_count_sum}");
}
