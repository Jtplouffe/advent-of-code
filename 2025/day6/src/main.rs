use crate::worksheet::Worksheet;

mod operator;
mod worksheet;

fn main() {
    let input = include_str!("./input").trim();

    let worksheet_horizontal_operands = Worksheet::new_horizontal_operands(input);
    println!("Part 1: {}", worksheet_horizontal_operands.grand_total());

    let worksheet_vertical_operands = Worksheet::new_vertical_operands(input);
    println!("Part 2: {}", worksheet_vertical_operands.grand_total());
}
