use crate::workflow_rule::{WorkflowRule, WorkflowRuleOperator};

pub struct PartRatingConstraints {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl PartRatingConstraints {
    pub fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    pub fn is_possible(&self) -> bool {
        self.x.1 >= self.x.0 && self.m.1 >= self.m.0 && self.a.1 >= self.a.0 && self.s.1 >= self.s.0
    }

    pub fn combination_count(&self) -> u128 {
        if !self.is_possible() {
            0
        } else {
            (self.x.1 - self.x.0 + 1) as u128
                * (self.m.1 - self.m.0 + 1) as u128
                * (self.a.1 - self.a.0 + 1) as u128
                * (self.s.1 - self.s.0 + 1) as u128
        }
    }

    pub fn constraint_for_rule(&self, rule: &WorkflowRule) -> (Self, Self) {
        match rule.variable {
            "x" => match rule.operator {
                WorkflowRuleOperator::GreaterThan => {
                    let new_min = (rule.value + 1).max(self.x.0);

                    (
                        Self {
                            x: (new_min, self.x.1),
                            m: self.m,
                            a: self.a,
                            s: self.s,
                        },
                        Self {
                            x: (self.x.0, rule.value),
                            m: self.m,
                            a: self.a,
                            s: self.s,
                        },
                    )
                }
                WorkflowRuleOperator::LessThan => {
                    let new_max = (rule.value - 1).min(self.x.1);

                    (
                        Self {
                            x: (self.x.0, new_max),
                            m: self.m,
                            a: self.a,
                            s: self.s,
                        },
                        Self {
                            x: (rule.value, self.x.1),
                            m: self.m,
                            a: self.a,
                            s: self.s,
                        },
                    )
                }
            },
            "m" => match rule.operator {
                WorkflowRuleOperator::GreaterThan => {
                    let new_min = (rule.value + 1).max(self.m.0);

                    (
                        Self {
                            x: self.x,
                            m: (new_min, self.m.1),
                            a: self.a,
                            s: self.s,
                        },
                        Self {
                            x: self.x,
                            m: (self.m.0, rule.value),
                            a: self.a,
                            s: self.s,
                        },
                    )
                }
                WorkflowRuleOperator::LessThan => {
                    let new_max = (rule.value - 1).min(self.m.1);

                    (
                        Self {
                            x: self.x,
                            m: (self.m.0, new_max),
                            a: self.a,
                            s: self.s,
                        },
                        Self {
                            x: self.x,
                            m: (rule.value, self.m.1),
                            a: self.a,
                            s: self.s,
                        },
                    )
                }
            },
            "a" => match rule.operator {
                WorkflowRuleOperator::GreaterThan => {
                    let new_min = (rule.value + 1).max(self.a.0);

                    (
                        Self {
                            x: self.x,
                            m: self.m,
                            a: (new_min, self.a.1),
                            s: self.s,
                        },
                        Self {
                            x: self.x,
                            m: self.m,
                            a: (self.a.0, rule.value),
                            s: self.s,
                        },
                    )
                }
                WorkflowRuleOperator::LessThan => {
                    let new_max = (rule.value - 1).min(self.a.1);

                    (
                        Self {
                            x: self.x,
                            m: self.m,
                            a: (self.a.0, new_max),
                            s: self.s,
                        },
                        Self {
                            x: self.x,
                            m: self.m,
                            a: (rule.value, self.a.1),
                            s: self.s,
                        },
                    )
                }
            },
            "s" => match rule.operator {
                WorkflowRuleOperator::GreaterThan => {
                    let new_min = (rule.value + 1).max(self.s.0);

                    (
                        Self {
                            x: self.x,
                            m: self.m,
                            a: self.a,
                            s: (new_min, self.s.1),
                        },
                        Self {
                            x: self.x,
                            m: self.m,
                            a: self.a,
                            s: (self.s.0, rule.value),
                        },
                    )
                }
                WorkflowRuleOperator::LessThan => {
                    let new_max = (rule.value - 1).min(self.s.1);

                    (
                        Self {
                            x: self.x,
                            m: self.m,
                            a: self.a,
                            s: (self.s.0, new_max),
                        },
                        Self {
                            x: self.x,
                            m: self.m,
                            a: self.a,
                            s: (rule.value, self.s.1),
                        },
                    )
                }
            },
            _ => unreachable!(),
        }
    }
}
