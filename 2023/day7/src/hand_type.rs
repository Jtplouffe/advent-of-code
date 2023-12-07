use std::collections::HashMap;

use crate::card::Card;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn from_hand_cards(cards: &[Card]) -> Self {
        let mut card_counts = HashMap::<&Card, usize>::new();

        for card in cards {
            *card_counts.entry(card).or_default() += 1;
        }

        let has_card_count = |count: usize| card_counts.values().any(|&value| value == count);

        if has_card_count(5) {
            return Self::FiveOfAKind;
        }

        if has_card_count(4) {
            return Self::FourOfAKind;
        }

        if has_card_count(3) {
            if has_card_count(2) {
                return Self::FullHouse;
            } else {
                return Self::ThreeOfAKind;
            }
        }

        let pairs_count = card_counts.values().filter(|&&count| count == 2).count();
        if pairs_count == 2 {
            return Self::TwoPair;
        } else if pairs_count == 1 {
            return Self::OnePair;
        }

        return Self::HighCard;
    }
}
