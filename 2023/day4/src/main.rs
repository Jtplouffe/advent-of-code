use crate::card_stack::CardStack;

mod card;
mod card_stack;

fn main() {
    let input = include_str!("./input");

    let card_stack = CardStack::from(input);

    let cards_points_sum = card_stack.points_sum();
    println!("Part 1: {cards_points_sum}");

    let final_card_count = card_stack.final_card_count();
    println!("Part 2: {final_card_count}");
}
