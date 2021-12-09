use std::str::FromStr;

fn main() {
    let input = include_str!("input");
    let entries: Vec<_> = input.lines().map(SignalEntry::from).collect();

    println!(
        "Part one: {}",
        entries
            .iter()
            .map(SignalEntry::pre_decoded_output_count)
            .sum::<usize>()
    );

    println!(
        "Part two: {}",
        entries.iter().map(SignalEntry::output_value).sum::<usize>()
    );
}

struct SignalEntry {
    inputs: Vec<SignalPattern>,
    outputs: Vec<SignalPattern>,
}

impl SignalEntry {
    fn pre_decoded_output_count(&self) -> usize {
        self.outputs
            .iter()
            .filter(|o| o.is_one() || o.is_four() || o.is_seven() || o.is_eight())
            .count()
    }

    fn output_value(&self) -> usize {
        let one = self.inputs.iter().find(|o| o.is_one()).unwrap();
        let four = self.inputs.iter().find(|o| o.is_four()).unwrap();

        let mut numbers_vec = vec![""; 10];

        for input in &self.inputs {
            let number = if input.is_one() {
                1
            } else if input.is_four() {
                4
            } else if input.is_seven() {
                7
            } else if input.is_eight() {
                8
            } else {
                match input.pattern.len() {
                    5 => {
                        if one.pattern.chars().all(|c| input.pattern.contains(c)) {
                            3
                        } else if four
                            .pattern
                            .chars()
                            .filter(|&c| input.pattern.contains(c))
                            .count()
                            == 3
                        {
                            5
                        } else {
                            2
                        }
                    }
                    6 => {
                        if four.pattern.chars().all(|c| input.pattern.contains(c)) {
                            9
                        } else if one.pattern.chars().all(|c| input.pattern.contains(c)) {
                            0
                        } else {
                            6
                        }
                    }
                    _ => panic!("Unknown pattern: {}", input.pattern),
                }
            };

            numbers_vec[number] = &input.pattern;
        }

        let mut number = String::new();
        for output in &self.outputs {
            number += &numbers_vec
                .iter()
                .position(|&num| {
                    output.pattern.len() == num.len()
                        && output.pattern.chars().all(|o| num.contains(o))
                })
                .unwrap()
                .to_string();
        }

        usize::from_str(&number).unwrap()
    }
}

impl From<&str> for SignalEntry {
    fn from(s: &str) -> Self {
        let (inputs, outputs) = s.split_once(" | ").unwrap();
        let inputs: Vec<_> = inputs.split(" ").map(SignalPattern::new).collect();
        let outputs: Vec<_> = outputs.split(" ").map(SignalPattern::new).collect();

        Self { inputs, outputs }
    }
}

#[derive(Debug)]
struct SignalPattern {
    pattern: String,
}

impl SignalPattern {
    fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
        }
    }

    fn is_one(&self) -> bool {
        self.pattern.len() == 2
    }

    fn is_four(&self) -> bool {
        self.pattern.len() == 4
    }

    fn is_seven(&self) -> bool {
        self.pattern.len() == 3
    }

    fn is_eight(&self) -> bool {
        self.pattern.len() == 7
    }
}
