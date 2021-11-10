use std::str::FromStr;
use std::collections::HashMap;

pub struct BagContent {
    pub count: u8,
    pub color: String,
}

impl BagContent {
    pub fn contains(&self, bag_color: &str, bags: &HashMap<String, Vec<BagContent>>) -> bool {
        self.color == bag_color || bags.get(&self.color).unwrap().iter().any(|bag| bag.contains(bag_color, bags))
    }

    pub fn count_bags_inside(&self, bags: &HashMap<String, Vec<BagContent>>, count: usize) -> usize {
        let content = bags.get(&self.color).unwrap();
        if content.is_empty() {
            count
        } else {
            content.iter().map(|bag| bag.count_bags_inside(bags, bag.count as usize * count)).sum::<usize>() + count
        }
    }
}

impl FromStr for BagContent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, color) = s.split_once(" ").unwrap();

        Ok(Self {
            count: count.parse::<u8>().unwrap(),
            color: color.to_string(),
        })
    }
}