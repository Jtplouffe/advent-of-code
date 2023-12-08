use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn cmp_with_jokers(&self, other: &Self) -> Ordering {
        if self == &Self::Jack && other != &Self::Jack {
            Ordering::Less
        } else if other == &Self::Jack && self != &Self::Jack {
            Ordering::Greater
        } else {
            self.cmp(other)
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}
