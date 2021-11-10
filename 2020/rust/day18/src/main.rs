use crate::expression::Expression;

mod expression;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let mut expressions: Vec<Expression> = input.lines().map(|l| l.into()).collect();

    println!("Part 1: {}", expressions.iter_mut().map(|e| e.evaluate()).sum::<usize>());
    println!("Part 2: {}", expressions.iter_mut().map(|e| e.evaluate_advanced()).sum::<usize>());
}