use camel_cards::CamelCards;

mod camel_cards;
mod card;
mod hand;
mod hand_type;

fn main() {
    let input = include_str!("./input");

    let mut camel_cards = CamelCards::from(input);
    camel_cards.sort_hands();

    let total_winnings = camel_cards.total_winnings();
    println!("Part 1: {total_winnings}");
}
