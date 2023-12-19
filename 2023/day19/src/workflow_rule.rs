use crate::{part_ratings::PartRatings, workflow_destination::WorkflowDestination};

pub enum WorkflowRuleOperator {
    GreaterThan,
    LessThan,
}

pub struct WorkflowRule<'a> {
    pub variable: &'a str,
    pub operator: WorkflowRuleOperator,
    pub value: u32,
    pub destination: WorkflowDestination<'a>,
}

impl<'a> WorkflowRule<'a> {
    pub fn matches(&self, part_ratings: &PartRatings) -> bool {
        let value = part_ratings.get_value(self.variable);

        match self.operator {
            WorkflowRuleOperator::LessThan => value < self.value,
            WorkflowRuleOperator::GreaterThan => value > self.value,
        }
    }
}

impl<'a> From<&'a str> for WorkflowRule<'a> {
    fn from(value: &'a str) -> Self {
        let (rule, destination) = value.split_once(':').unwrap();
        let destination = WorkflowDestination::from(destination);

        if let Some((variable, value)) = rule.split_once('<') {
            Self {
                variable,
                operator: WorkflowRuleOperator::LessThan,
                value: value.parse().unwrap(),
                destination,
            }
        } else if let Some((variable, value)) = rule.split_once('>') {
            Self {
                variable,
                operator: WorkflowRuleOperator::GreaterThan,
                value: value.parse().unwrap(),
                destination,
            }
        } else {
            unreachable!()
        }
    }
}
