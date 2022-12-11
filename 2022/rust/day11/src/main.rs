use crate::keep_away::KeepAway;

mod keep_away;
mod monkey;

fn main() {
    let input = include_str!("./input");

    let mut keep_away_1 = KeepAway::new(input, 3);
    keep_away_1.execute_rounds(20);
    println!("Part 1: {}", keep_away_1.monkey_business());

    let mut keep_away_2 = KeepAway::new(input, 0);
    keep_away_2.execute_rounds(10000);
    println!("Part 1: {}", keep_away_2.monkey_business());
}
