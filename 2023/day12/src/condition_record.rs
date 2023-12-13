use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::spring_condition::SpringCondition;

pub struct ConditionRecord {
    spring_conditions: Vec<SpringCondition>,
    damaged_spring_groups: Vec<usize>,
}

impl ConditionRecord {
    pub fn possible_unfolded_arrangement_count(&self, fold_count: usize) -> usize {
        let unfolded_spring_conditions = self.unfold_spring_conditions(fold_count);
        let unfolded_damaged_spring_groups = self.unfold_damaged_spring_groups(fold_count);

        let unknown_count = unfolded_spring_conditions
            .iter()
            .filter(|&&spring_condition| spring_condition == SpringCondition::Unknown)
            .count();

        (0..0x1 << unknown_count)
            .into_par_iter()
            .filter(|&flag| {
                let expanded_spring_conditions =
                    Self::generate_expanded_spring_conditions_from_flag(
                        &unfolded_spring_conditions,
                        flag,
                    );

                Self::is_expanded_spring_conditions_possible(
                    &expanded_spring_conditions,
                    &unfolded_damaged_spring_groups,
                )
            })
            .count()
    }

    fn is_expanded_spring_conditions_possible(
        expanded_spring_conditions: &[SpringCondition],
        damaged_spring_groups: &[usize],
    ) -> bool {
        let mut expanded_damaged_spring_groups = Vec::with_capacity(damaged_spring_groups.len());

        let mut current_group_size = 0;
        for spring_condition in expanded_spring_conditions {
            match spring_condition {
                SpringCondition::Operational => {
                    if current_group_size == 0 {
                        continue;
                    }

                    expanded_damaged_spring_groups.push(current_group_size);
                    current_group_size = 0;
                }
                SpringCondition::Damaged => current_group_size += 1,
                SpringCondition::Unknown => unreachable!(),
            }
        }

        if current_group_size != 0 {
            expanded_damaged_spring_groups.push(current_group_size);
        }

        expanded_damaged_spring_groups.eq(damaged_spring_groups)
    }

    fn generate_expanded_spring_conditions_from_flag(
        spring_conditions: &[SpringCondition],
        flag: usize,
    ) -> Vec<SpringCondition> {
        let mut expanded_spring_condition = Vec::with_capacity(spring_conditions.len());

        let mut unknown_index = 0;
        for spring_condition in spring_conditions {
            let spring_condition = match spring_condition {
                SpringCondition::Unknown => {
                    let spring_condition = if flag & (0x1 << unknown_index) == 0 {
                        SpringCondition::Operational
                    } else {
                        SpringCondition::Damaged
                    };

                    unknown_index += 1;
                    spring_condition
                }
                &spring_condition => spring_condition,
            };
            expanded_spring_condition.push(spring_condition);
        }

        expanded_spring_condition
    }

    fn unfold_spring_conditions(&self, fold_count: usize) -> Vec<SpringCondition> {
        let mut unfolded_spring_conditions =
            Vec::with_capacity(self.spring_conditions.len() * 5 + 4);

        for i in 0..fold_count {
            unfolded_spring_conditions.extend(&self.spring_conditions);
            if i < fold_count - 1 {
                unfolded_spring_conditions.push(SpringCondition::Unknown);
            }
        }

        unfolded_spring_conditions
    }

    fn unfold_damaged_spring_groups(&self, fold_count: usize) -> Vec<usize> {
        self.damaged_spring_groups.repeat(fold_count)
    }
}

impl From<&str> for ConditionRecord {
    fn from(value: &str) -> Self {
        let (spring_conditions, damaged_spring_groups) = value.split_once(' ').unwrap();

        let spring_conditions = spring_conditions
            .chars()
            .map(SpringCondition::from)
            .collect();
        let damaged_spring_groups = damaged_spring_groups
            .split(',')
            .map(|group| group.parse().unwrap())
            .collect();

        Self {
            spring_conditions,
            damaged_spring_groups,
        }
    }
}
