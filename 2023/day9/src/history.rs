pub struct History {
    values: Vec<i32>,
}

impl History {
    pub fn extrapolate_next_value(&self) -> i32 {
        let deltas = self.expand_deltas();

        deltas
            .iter()
            .map(|values| values.last().unwrap())
            .sum::<i32>()
            + self.values.last().unwrap()
    }

    pub fn extrapolate_previous_value(&self) -> i32 {
        let deltas = self.expand_deltas();

        self.values.first().unwrap()
            - deltas
                .iter()
                .rev()
                .fold(0, |acc, deltas| deltas.first().unwrap() - acc)
    }

    fn expand_deltas(&self) -> Vec<Vec<i32>> {
        let mut deltas = Vec::<Vec<_>>::new();

        loop {
            let current_deltas = match deltas.last() {
                Some(values) => Self::values_deltas(values),
                None => Self::values_deltas(&self.values),
            };
            let all_zeros = current_deltas.iter().all(|&delta| delta == 0);

            deltas.push(current_deltas);

            if all_zeros {
                break;
            }
        }

        deltas
    }

    fn values_deltas(values: &[i32]) -> Vec<i32> {
        let mut deltas = Vec::with_capacity(values.len() - 1);

        for i in 1..values.len() {
            deltas.push(values[i] - values[i - 1]);
        }

        deltas
    }
}

impl From<&str> for History {
    fn from(value: &str) -> Self {
        let values = value
            .split_whitespace()
            .map(|value| value.parse().expect("expected number"))
            .collect();

        Self { values }
    }
}
