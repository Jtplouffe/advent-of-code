use std::collections::HashMap;

use crate::card::Card;

pub struct CardStack {
    cards: Vec<Card>,
}

impl CardStack {
    pub fn points_sum(&self) -> u32 {
        self.cards.iter().map(Card::points).sum()
    }

    pub fn final_card_count(&self) -> u32 {
        let mut won_cards_quantity = HashMap::<usize, u32>::new();

        for card in &self.cards {
            let quantity = match won_cards_quantity.get(&card.id) {
                Some(quantity) => quantity + 1,
                None => 1,
            };

            let matching_number_count = card.matching_number_count();
            let last_card_id = self.cards.last().unwrap().id;
            for id in card.id + 1..=(card.id + matching_number_count).min(last_card_id) {
                let entry = won_cards_quantity.entry(id).or_default();
                *entry += quantity;
            }
        }

        self.cards.len() as u32 + won_cards_quantity.values().sum::<u32>()
    }
}

impl From<&str> for CardStack {
    fn from(value: &str) -> Self {
        let cards = value.lines().map(Card::from).collect();
        Self { cards }
    }
}
