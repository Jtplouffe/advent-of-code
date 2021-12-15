use std::collections::HashMap;

pub struct Polymerizater {
    polymer_template: String,
    polymer_template_map: HashMap<(char, char), usize>,
    polymer_pair_insertion_rules: HashMap<(char, char), char>,
}

impl Polymerizater {
    pub fn run_step(&mut self) {
        let mut next_template = HashMap::new();

        for (&(a, b), count) in &self.polymer_template_map {
            let result = self.polymer_pair_insertion_rules[&(a, b)];
            *next_template.entry((a, result)).or_insert(0) += count;
            *next_template.entry((result, b)).or_insert(0) += count;
        }

        self.polymer_template_map = next_template;
    }

    pub fn run_steps(&mut self, count: usize) {
        for _ in 0..count {
            self.run_step();
        }
    }

    pub fn diff_most_common_and_least_common_count(&self) -> usize {
        let mut count_map = HashMap::<char, usize>::new();
        for (&(a, _), count) in &self.polymer_template_map {
            *count_map.entry(a).or_insert(0) += count;
        }
        *count_map
            .entry(self.polymer_template.chars().last().unwrap())
            .or_insert(0) += 1;

        let min = count_map.values().min().unwrap();
        let max = count_map.values().max().unwrap();
        max - min
    }
}

impl From<&str> for Polymerizater {
    fn from(s: &str) -> Self {
        let (template, rules) = s.split_once("\n\n").unwrap();
        let polymer_template = template.to_string();

        let mut polymer_template_map = HashMap::new();
        for (&a, &b) in template.chars().collect::<Vec<_>>()[0..template.len() - 1]
            .iter()
            .zip(template.chars().collect::<Vec<_>>()[1..template.len()].iter())
        {
            *polymer_template_map.entry((a, b)).or_insert(0) += 1;
        }

        let polymer_pair_insertion_rules = rules
            .lines()
            .map(|l| {
                let (pair, result) = l.split_once(" -> ").unwrap();
                (
                    (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()),
                    result.chars().nth(0).unwrap(),
                )
            })
            .collect();

        Self {
            polymer_template,
            polymer_template_map,
            polymer_pair_insertion_rules,
        }
    }
}
