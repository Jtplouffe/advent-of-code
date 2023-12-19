use std::collections::{HashMap, VecDeque};

use crate::{
    part_rating_constraints::PartRatingConstraints, part_ratings::PartRatings, workflow::Workflow,
    workflow_destination::WorkflowDestination,
};

const STARTING_WORKFLOW_NAME: &str = "in";

pub struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
}

impl<'a> System<'a> {
    pub fn is_part_ratings_accepted(&self, part_ratings: &PartRatings) -> bool {
        let mut current_workflow = &self.workflows[STARTING_WORKFLOW_NAME];

        loop {
            match current_workflow.part_ratings_destination(part_ratings) {
                WorkflowDestination::Accept => return true,
                WorkflowDestination::Reject => return false,
                WorkflowDestination::Workflow(workflow) => {
                    current_workflow = &self.workflows[workflow]
                }
            }
        }
    }

    pub fn acceptable_distinct_combination_count(&self) -> u128 {
        let mut acceptable_constraints = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back((
            WorkflowDestination::Workflow(STARTING_WORKFLOW_NAME),
            PartRatingConstraints::new(),
        ));

        while let Some((from_destination, mut constraints)) = queue.pop_front() {
            let workflow = match from_destination {
                WorkflowDestination::Accept => {
                    acceptable_constraints.push(constraints);
                    continue;
                }
                WorkflowDestination::Reject => {
                    continue;
                }
                WorkflowDestination::Workflow(workflow) => &self.workflows[workflow],
            };

            for rule in &workflow.rules {
                let (constrained_part_ratings, inversed_constrained_part_ratings) =
                    constraints.constraint_for_rule(rule);

                if constrained_part_ratings.is_possible() {
                    queue.push_back((rule.destination, constrained_part_ratings));
                }

                constraints = inversed_constrained_part_ratings;
                if !constraints.is_possible() {
                    break;
                }
            }

            if constraints.is_possible() {
                queue.push_back((workflow.fallback_destination, constraints));
            }
        }

        acceptable_constraints
            .iter()
            .map(PartRatingConstraints::combination_count)
            .sum()
    }
}

impl<'a> From<&'a str> for System<'a> {
    fn from(value: &'a str) -> Self {
        let workflows = value
            .lines()
            .map(Workflow::from)
            .map(|workflow| (workflow.name, workflow))
            .collect();

        Self { workflows }
    }
}
