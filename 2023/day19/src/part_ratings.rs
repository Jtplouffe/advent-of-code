pub struct PartRatings {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl PartRatings {
    pub fn get_value(&self, variable: &str) -> u32 {
        match variable {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!(),
        }
    }

    pub fn values_sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for PartRatings {
    fn from(value: &str) -> Self {
        let ratings = value.replace(['{', '}'], "");
        let mut ratings = ratings.split(',').map(|rating| {
            let (variable, value) = rating.split_once('=').unwrap();
            (variable, value.parse().unwrap())
        });

        let mut find_variable_value = |variable: &str| -> u32 {
            ratings
                .find_map(
                    |(var, value)| {
                        if var == variable {
                            Some(value)
                        } else {
                            None
                        }
                    },
                )
                .unwrap()
        };

        Self {
            x: find_variable_value("x"),
            m: find_variable_value("m"),
            a: find_variable_value("a"),
            s: find_variable_value("s"),
        }
    }
}
