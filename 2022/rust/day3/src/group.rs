use crate::rucksack::Item;
use crate::Rucksack;

pub struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    pub fn new(rucksacks: Vec<Rucksack>) -> Self {
        Self { rucksacks }
    }

    pub fn badge_item(&self) -> &Item {
        for item in self.rucksacks[0].items() {
            if self.rucksack_and_following_has_item(1, item) {
                return item;
            }
        }

        unreachable!()
    }

    fn rucksack_and_following_has_item(&self, rucksack_index: usize, item: &Item) -> bool {
        self.rucksacks[rucksack_index].has_item(item)
            && (rucksack_index == self.rucksacks.len() - 1
                || self.rucksack_and_following_has_item(rucksack_index + 1, item))
    }
}
