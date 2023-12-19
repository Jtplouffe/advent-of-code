use crate::{
    part_ratings::PartRatings, workflow_destination::WorkflowDestination,
    workflow_rule::WorkflowRule,
};

pub struct Workflow<'a> {
    pub name: &'a str,
    pub rules: Vec<WorkflowRule<'a>>,
    pub fallback_destination: WorkflowDestination<'a>,
}

impl<'a> Workflow<'a> {
    pub fn part_ratings_destination(&self, part_ratings: &PartRatings) -> WorkflowDestination {
        for rule in &self.rules {
            if rule.matches(part_ratings) {
                return rule.destination;
            }
        }

        self.fallback_destination
    }
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let (name, rules) = value.split_once('{').unwrap();
        let rules = rules.split('}').next().unwrap();

        let fallback_destination = WorkflowDestination::from(rules.split(',').next_back().unwrap());

        let rules = rules
            .split(',')
            .filter_map(|rule| {
                if rule.contains(':') {
                    Some(WorkflowRule::from(rule))
                } else {
                    None
                }
            })
            .collect();

        Self {
            name,
            rules,
            fallback_destination,
        }
    }
}
