use itertools::Itertools;
use std::collections::HashMap;

pub struct Rucksack {
    first_compartment: HashMap<Item, usize>,
    second_compartment: HashMap<Item, usize>,
}

impl<'a> Rucksack {
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.first_compartment
            .keys()
            .merge(self.second_compartment.keys())
    }

    pub fn error_item(&self) -> Item {
        for first_compartment_item in self.first_compartment.keys() {
            if self.second_compartment.contains_key(first_compartment_item) {
                return *first_compartment_item;
            }
        }

        unreachable!()
    }

    pub fn has_item(&self, item: &Item) -> bool {
        self.first_compartment.contains_key(item) || self.second_compartment.contains_key(item)
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let (first_compartment_str, second_compartment_str) = s.split_at(s.len() / 2);

        let mut first_compartment = HashMap::new();
        for c in first_compartment_str.chars() {
            *first_compartment.entry(c.into()).or_insert(0) += 1;
        }

        let mut second_compartment = HashMap::new();
        for c in second_compartment_str.chars() {
            *second_compartment.entry(c.into()).or_insert(0) += 1;
        }

        Self {
            first_compartment,
            second_compartment,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct Item(char);

impl Item {
    pub fn priority(&self) -> usize {
        let char_code = self.0 as usize;

        if char_code >= 65 && char_code <= 90 {
            char_code - 38
        } else if char_code >= 97 && char_code <= 122 {
            char_code - 96
        } else {
            unreachable!()
        }
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Self(c)
    }
}
