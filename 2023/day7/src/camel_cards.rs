use crate::hand::Hand;

pub struct CamelCards {
    pub hands: Vec<Hand>,
}

impl CamelCards {
    pub fn sort_hands(&mut self) {
        self.hands.sort();
    }

    pub fn sort_hands_with_jokers(&mut self) {
        self.hands.sort_by(|a, b| a.cmp_with_jokers(b));
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
