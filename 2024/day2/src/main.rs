use std::ops::Sub;

fn main() {
    let input = include_str!("./input");

    let reports: Vec<_> = input.trim_end().lines().map(Report::from).collect();

    let safe_report_count = reports.iter().filter(|report| report.is_safe()).count();
    println!("Part 1: {safe_report_count}");

    let safe_report_count_with_problem_dampener = reports
        .iter()
        .filter(|report| report.is_safe_with_problem_dampener())
        .count();
    println!("Part 2: {safe_report_count_with_problem_dampener}");
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        Self::inner_is_safe(&self.levels)
    }

    pub fn is_safe_with_problem_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(i);

            if Self::inner_is_safe(&levels) {
                return true;
            }
        }

        false
    }

    fn inner_is_safe(levels: &[i32]) -> bool {
        let signum = match levels[1].sub(levels[0]).signum() {
            0 => return false,
            sign => sign,
        };

        for window in levels.windows(2) {
            let left = window[0];
            let right = window[1];

            if (signum == -1 && left <= right)
                || (signum == 1 && left >= right)
                || left.sub(right).abs() > 3
            {
                return false;
            }
        }

        true
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let levels = value
            .split(" ")
            .map(|level| level.parse().unwrap())
            .collect();

        Self { levels }
    }
}
