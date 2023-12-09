use crate::history::History;

pub struct OasisReport {
    histories: Vec<History>,
}

impl OasisReport {
    pub fn extrapolated_next_values_sum(&self) -> i32 {
        self.histories
            .iter()
            .map(History::extrapolate_next_value)
            .sum()
    }

    pub fn extrapolated_previous_values_sum(&self) -> i32 {
        self.histories
            .iter()
            .map(History::extrapolate_previous_value)
            .sum()
    }
}

impl From<&str> for OasisReport {
    fn from(value: &str) -> Self {
        let histories = value.lines().map(History::from).collect();

        Self { histories }
    }
}
