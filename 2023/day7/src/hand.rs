use std::cmp::Ordering;

use crate::{card::Card, hand_type::HandType};

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,

    pub bid: usize,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(" ").expect("expected white space");

        let cards: Vec<_> = cards.chars().map(Card::from).collect();
        let hand_type = HandType::from_hand_cards(&cards);

        let bid = bid.parse().expect("expected bid number");

        Self {
            cards,
            hand_type,
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
