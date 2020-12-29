use std::collections::HashMap;

pub struct Rule {
    pub id: usize,
    value: Option<String>,
    sub_rules: Option<Vec<Vec<usize>>>,
}

impl Rule {
    pub fn valid(&self, data: &Vec<&str>, rules: &HashMap<usize, Rule>) -> usize {
        let matches = self.matches(rules);

        matches.iter().filter(|m| data.contains(&m.as_str())).count()
    }

    fn matches(&self, rules: &HashMap<usize, Rule>) -> Vec<String> {
        let mut matches = Vec::new();

        if let Some(value) = &self.value {
            matches.push(value.clone());
        } else if let Some(sub_rules) = &self.sub_rules {
            for sub_rule in sub_rules {
                let items: Vec<Vec<String>> = sub_rule.iter().map(|id| rules[id].matches(rules)).collect();
                let combinations = Self::all_combinations(items);
                matches.extend(combinations);
            }
        }

        matches
    }

    fn all_combinations(data: Vec<Vec<String>>) -> Vec<String> {
        let mut combinations = Vec::new();

        let mut temp_combinations = data[0].clone();

        if data.len() == 1 {
            combinations = temp_combinations;
        } else {
            for (index, d) in data[1..].iter().enumerate().map(|(i, d)| (i + 1, d)) {
                for i in d {
                    let mut new_combinations = Vec::new();

                    for c in &temp_combinations {
                        if index + 1 == data.len() {
                            combinations.push(c.to_owned() + i);
                        } else {
                            new_combinations.push(c.to_owned() + i);
                        }
                    }
                    temp_combinations.extend(new_combinations);
                }
            }
        }

        combinations
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let (id, rule) = s.split_once(": ")
            .and_then(|(i, r)| Some((i.parse::<usize>().unwrap(), r)))
            .unwrap();

        let (value, sub_rules): (Option<String>, Option<Vec<Vec<usize>>>) = if rule.contains('"') {
            (
                Some(rule.replace('"', "")),
                None
            )
        } else {
            (
                None,
                Some(rule.split(" | ").map(|s| s.split(" ").map(|e| e.parse::<usize>().unwrap()).collect()).collect())
            )
        };

        Self { id, value, sub_rules }
    }
}