use std::cmp::Ordering;

use crate::{card::Card, hand_type::HandType};

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    hand_type_with_jokers: HandType,

    pub bid: usize,
}

impl Hand {
    pub fn cmp_with_jokers(&self, other: &Self) -> Ordering {
        match self.hand_type_with_jokers.cmp(&other.hand_type_with_jokers) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(
                    |(card_left, card_right)| match card_left.cmp_with_jokers(card_right) {
                        Ordering::Equal => None,
                        ordering => Some(ordering),
                    },
                )
                .unwrap_or(Ordering::Equal),
            ordering => ordering,
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(' ').expect("expected white space");

        let cards: Vec<_> = cards.chars().map(Card::from).collect();
        let hand_type = HandType::from_hand_cards(&cards);
        let hand_type_with_jokers = HandType::from_hard_cards_with_jokers(&cards);

        let bid = bid.parse().expect("expected bid number");

        Self {
            cards,
            hand_type,
            hand_type_with_jokers,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
