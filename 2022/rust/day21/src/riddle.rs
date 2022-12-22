use std::collections::HashMap;

use crate::monkey::{Monkey, MonkeyJob};

const ROOT: &str = "root";
const HUMAN: &str = "humn";

pub struct Riddle<'a> {
    monkeys: HashMap<&'a str, Monkey<'a>>,
}

impl<'a> Riddle<'a> {
    pub fn solve_part_1(&mut self) -> i64 {
        self.get_monkey_value(ROOT)
    }

    // Takes a very, very, very long time to solve
    pub fn solve_part_2(&mut self) -> i64 {
        let (left_monkey_constant_value, right_monkey_constant_value) =
            match &self.monkeys[ROOT].job.clone() {
                MonkeyJob::Arithmetic((left_monkey_name, _, right_monkey_name)) => {
                    let left_monkey_uses_human = self.monkey_uses_monkey(left_monkey_name, HUMAN);
                    let right_monkey_uses_human = self.monkey_uses_monkey(right_monkey_name, HUMAN);

                    let left_monkey_constant_value = if left_monkey_uses_human {
                        None
                    } else {
                        Some(self.get_monkey_value(left_monkey_name))
                    };

                    let right_monkey_constant_value = if right_monkey_uses_human {
                        None
                    } else {
                        Some(self.get_monkey_value(right_monkey_name))
                    };

                    (left_monkey_constant_value, right_monkey_constant_value)
                }
                _ => unreachable!(),
            };

        for i in 0.. {
            self.monkeys.get_mut(HUMAN).unwrap().job = MonkeyJob::Yell(i);

            match &self.monkeys[ROOT].job.clone() {
                MonkeyJob::Arithmetic((left_monkey_name, _, right_monkey_name)) => {
                    let left_monkey_value = if let Some(constant_value) = left_monkey_constant_value
                    {
                        constant_value
                    } else {
                        self.get_monkey_value(left_monkey_name)
                    };

                    let right_monkey_value =
                        if let Some(constant_value) = right_monkey_constant_value {
                            constant_value
                        } else {
                            self.get_monkey_value(right_monkey_name)
                        };

                    if left_monkey_value == right_monkey_value {
                        return i;
                    }
                }
                _ => unreachable!(),
            };
        }

        unreachable!()
    }

    fn get_monkey_value(&mut self, name: &'a str) -> i64 {
        match &self.monkeys[name].job.clone() {
            MonkeyJob::Yell(value) => *value,
            MonkeyJob::Arithmetic((left_monkey_name, operation, right_monkey_name)) => {
                let left_value = self.get_monkey_value(&left_monkey_name);
                let right_value = self.get_monkey_value(&right_monkey_name);
                operation.solve(left_value, right_value)
            }
        }
    }

    fn monkey_uses_monkey(&self, source_name: &'a str, target_name: &'a str) -> bool {
        match &self.monkeys[source_name].job {
            MonkeyJob::Yell(_) => false,
            MonkeyJob::Arithmetic((left_monkey_name, _, right_monkey_name)) => {
                *left_monkey_name == target_name
                    || *right_monkey_name == target_name
                    || self.monkey_uses_monkey(left_monkey_name, target_name)
                    || self.monkey_uses_monkey(right_monkey_name, target_name)
            }
        }
    }
}

impl<'a> From<&'a str> for Riddle<'a> {
    fn from(s: &'a str) -> Self {
        let monkeys = s
            .lines()
            .map(|line| {
                let monkey = Monkey::from(line);
                (monkey.name, monkey)
            })
            .collect();

        Self { monkeys }
    }
}
