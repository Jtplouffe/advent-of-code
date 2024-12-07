use crate::operators::Operators;

pub struct Equation {
    pub total: u64,
    values: Vec<u64>,
}

impl Equation {
    pub fn could_be_true(&self, operators: &[Operators]) -> bool {
        let mut operators_permutations = Vec::new();
        self.generate_operators_permutations(&mut operators_permutations, Vec::new(), operators);

        for operators in operators_permutations {
            let total = self.evaluate(&operators);
            if total == self.total {
                return true;
            }
        }

        false
    }

    fn evaluate(&self, operators: &[Operators]) -> u64 {
        assert_eq!(
            operators.len(),
            self.values.len() - 1,
            "Wrong number of operators"
        );

        let mut left = self.values[0];

        for (i, &right) in self.values[1..].iter().enumerate() {
            left = operators[i].evaluate(left, right);
        }

        left
    }

    fn generate_operators_permutations(
        &self,
        result: &mut Vec<Vec<Operators>>,
        base: Vec<Operators>,
        operators: &[Operators],
    ) {
        if base.len() == self.values.len() - 1 {
            result.push(base);
            return;
        }

        for &operator in operators {
            let mut current = base.clone();
            current.push(operator);
            self.generate_operators_permutations(result, current, operators);
        }
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (total, values) = value.split_once(": ").unwrap();

        let total = total.parse().unwrap();
        let values = values
            .split(' ')
            .map(|value| value.parse().unwrap())
            .collect();

        Self { total, values }
    }
}
