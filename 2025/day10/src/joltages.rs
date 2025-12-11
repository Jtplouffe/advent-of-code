#[derive(Default)]
pub struct Joltages {
    joltages: Vec<usize>,
}

impl Joltages {
    pub fn new(joltages: Vec<usize>) -> Self {
        Self { joltages }
    }

    pub fn is_over_requirements(&self, requirements: &Self) -> bool {
        for (index, requirement) in requirements.joltages.iter().enumerate() {
            let joltage = self.joltages.get(index).unwrap_or(&0);
            if joltage > requirement {
                return true;
            }
        }

        false
    }

    pub fn copy_increased(&self, increment_indexes: &[usize]) -> Self {
        let mut joltages = self.joltages.clone();

        for &index in increment_indexes {
            if let Some(joltage) = joltages.get_mut(index) {
                *joltage += 1;
            } else {
                let extension = std::iter::repeat(0).take(index - joltages.len() + 1);
                joltages.extend(extension);
                joltages[index] = 1;
            }
        }

        Self { joltages }
    }
}

impl PartialEq for Joltages {
    fn eq(&self, other: &Self) -> bool {
        let self_max_valued_index = (0..self.joltages.len())
            .rev()
            .find(|&index| self.joltages[index] != 0);
        let other_max_valued_index = (0..other.joltages.len())
            .rev()
            .find(|&index| other.joltages[index] != 0);

        if self_max_valued_index != other_max_valued_index
            || self_max_valued_index.is_none()
            || other_max_valued_index.is_none()
        {
            return false;
        }

        for index in 0..=self_max_valued_index.unwrap() {
            if self.joltages[index] != other.joltages[index] {
                return false;
            }
        }

        true
    }
}
