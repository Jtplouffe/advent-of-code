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

        let card_counts: Vec<_> = card_counts.into_values().collect();

        if card_counts.contains(&5) {
            return Self::FiveOfAKind;
        }

        if card_counts.contains(&4) {
            return Self::FourOfAKind;
        }

        if card_counts.contains(&3) {
            if card_counts.contains(&2) {
                return Self::FullHouse;
            } else {
                return Self::ThreeOfAKind;
            }
        }

        let pairs_count = card_counts.iter().filter(|&&count| count == 2).count();
        if pairs_count == 2 {
            Self::TwoPair
        } else if pairs_count == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }

    pub fn from_hard_cards_with_jokers(cards: &[Card]) -> Self {
        let mut card_counts = HashMap::<&Card, isize>::new();

        for card in cards {
            *card_counts.entry(card).or_default() += 1;
        }

        let mut joker_count = *card_counts.get(&Card::Jack).unwrap_or(&0);

        let mut card_counts: Vec<_> = card_counts
            .into_iter()
            .filter_map(|(card, count)| match card {
                Card::Jack => None,
                _ => Some(count),
            })
            .collect();

        let max_card_count = card_counts.iter().max().unwrap_or(&0);

        if max_card_count + joker_count >= 5 {
            return Self::FiveOfAKind;
        }

        if max_card_count + joker_count >= 4 {
            return Self::FourOfAKind;
        }

        if max_card_count + joker_count >= 3 {
            let index = card_counts.iter().enumerate().find_map(|(index, &count)| {
                if count >= 3 {
                    Some(index)
                } else {
                    None
                }
            });
            if let Some(index) = index {
                card_counts[index] -= 3;
            } else {
                let (max_count_index, max_count) = card_counts
                    .iter()
                    .copied()
                    .enumerate()
                    .reduce(|max, curr| if curr.1 > max.1 { curr } else { max })
                    .unwrap();
                card_counts[max_count_index] = 0;
                joker_count -= 3 - max_count;
            }

            if card_counts.iter().any(|&c| c >= 2 - joker_count) {
                return Self::FullHouse;
            } else {
                return Self::ThreeOfAKind;
            }
        }

        let pairs_count = card_counts.iter().filter(|&&count| count == 2).count() as isize;
        if pairs_count >= 2 - joker_count {
            Self::TwoPair
        } else if pairs_count >= 1 - joker_count {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}
