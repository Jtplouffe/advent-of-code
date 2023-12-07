use crate::hand::Hand;

pub struct CamelCards {
    hands: Vec<Hand>,
}

impl CamelCards {
    pub fn sort_hands(&mut self) {
        self.hands.sort();
    }

    pub fn total_winnings(&self) -> usize {
        self.hands
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index + 1))
            .sum()
    }
}

impl From<&str> for CamelCards {
    fn from(value: &str) -> Self {
        let hands = value.lines().map(Hand::from).collect();

        Self { hands }
    }
}
