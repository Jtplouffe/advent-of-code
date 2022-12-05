use std::collections::VecDeque;

use crate::Action;

#[derive(Clone)]
pub struct Port {
    stacks: Vec<VecDeque<Crate>>,
}

impl Port {
    pub fn apply_actions(&mut self, actions: &[Action], maintain_crate_order_for_action: bool) {
        for action in actions {
            let mut crates_to_move = VecDeque::with_capacity(action.move_count);
            for _ in 0..action.move_count {
                if maintain_crate_order_for_action {
                    crates_to_move.push_front(self.stacks[action.move_from_index].pop_front());
                } else {
                    crates_to_move.push_back(self.stacks[action.move_from_index].pop_front());
                }
            }

            for crate_to_move in crates_to_move {
                self.stacks[action.move_to_index].push_front(crate_to_move.unwrap());
            }
        }
    }

    pub fn formatted_top_crates(&self) -> String {
        let top_crates = self.top_crates();

        let mut value = String::with_capacity(top_crates.len());
        for top_crate in top_crates {
            value.push(top_crate.0);
        }

        value
    }

    pub fn top_crates(&self) -> Vec<&Crate> {
        let mut crates = Vec::with_capacity(self.stacks.len());
        for stack in &self.stacks {
            crates.push(stack.front().unwrap());
        }

        crates
    }
}

impl From<&str> for Port {
    fn from(s: &str) -> Self {
        let stack_count = s
            .lines()
            .last()
            .unwrap()
            .chars()
            .filter(|char| !char.is_whitespace())
            .count();

        let mut stacks = Vec::with_capacity(stack_count);

        let max_initial_stack_height = s.lines().count() - 1;
        for stack_index in 0..stack_count {
            let mut stack = VecDeque::new();
            let stack_char_index = stack_index * 4 + 1;

            for line_index in 0..max_initial_stack_height {
                let c = s
                    .lines()
                    .nth(line_index)
                    .unwrap()
                    .chars()
                    .nth(stack_char_index)
                    .unwrap();
                if !c.is_whitespace() {
                    stack.push_back(c.into());
                }
            }

            stacks.push(stack);
        }

        Self { stacks }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Crate(char);

impl Crate {
    fn new(c: char) -> Self {
        Self(c)
    }
}

impl From<char> for Crate {
    fn from(c: char) -> Self {
        Self::new(c)
    }
}
