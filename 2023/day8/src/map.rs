use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{direction::Direction, lcm::lcm};

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

pub struct Map<'a> {
    directions: Vec<Direction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    pub fn steps_to_end(&self) -> usize {
        self.steps_to_end_for_nodes(START_NODE, |node| node == END_NODE)
    }

    pub fn multi_node_steps_to_end_fast(&self) -> usize {
        let steps: Vec<_> = self
            .nodes
            .par_iter()
            .filter_map(|(node, _)| {
                if node.ends_with('A') {
                    Some(self.steps_to_end_for_nodes(node, |node| node.ends_with('Z')))
                } else {
                    None
                }
            })
            .collect();

        let mut max_steps = steps[0];
        for &step in &steps[1..] {
            max_steps = lcm(max_steps, step);
        }

        max_steps
    }

    fn steps_to_end_for_nodes<E>(&self, starting_node: &str, end_predicate: E) -> usize
    where
        E: Fn(&str) -> bool,
    {
        let mut steps = 0;

        let mut current_direction_index = 0;
        let mut current_node = starting_node;

        while !end_predicate(current_node) {
            let direction = &self.directions[current_direction_index];
            current_node = match direction {
                Direction::Left => self.nodes[current_node].0,
                Direction::Right => self.nodes[current_node].1,
            };

            steps += 1;
            current_direction_index = (current_direction_index + 1) % self.directions.len();
        }

        steps
    }
}

impl<'a> From<&'a str> for Map<'a> {
    fn from(value: &'a str) -> Self {
        let (directions, nodes) = value.split_once("\n\n").unwrap();
        let directions = directions.chars().map(Direction::from).collect();

        let nodes = nodes
            .lines()
            .map(|line| {
                let (source, targets) = line.split_once(" = (").unwrap();
                let (left, right) = targets.split(')').next().unwrap().split_once(", ").unwrap();
                (source, (left, right))
            })
            .collect();

        Self { directions, nodes }
    }
}
