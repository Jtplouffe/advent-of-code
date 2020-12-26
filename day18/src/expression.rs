pub struct Expression {
    exp: Vec<String>,
}

impl Expression {
    pub fn evaluate(&mut self) -> usize {
        let original_exp = self.exp.clone();
        while self.exp.len() > 1 {
            while let Some((opening_parenthesis, closing_parenthesis)) = self.deepest_parenthesis_index() {
                if closing_parenthesis - opening_parenthesis > 2 {
                    self.resolve_first_from_index(opening_parenthesis + 1);
                } else {
                    self.exp.remove(closing_parenthesis);
                    self.exp.remove(opening_parenthesis);
                }
            }

            self.resolve_first_from_index(0);
        }

        let result = self.exp[0].parse().unwrap();
        self.exp = original_exp;
        result
    }

    fn resolve_first_from_index(&mut self, index: usize) {
        let value = if self.exp[index + 1] == "+" {
            self.exp[index].parse::<usize>().unwrap() + self.exp[index + 2].parse::<usize>().unwrap()
        } else {
            self.exp[index].parse::<usize>().unwrap() * self.exp[index + 2].parse::<usize>().unwrap()
        };

        self.exp.insert(index, value.to_string());
        self.exp.drain(index + 1..=index + 3);
    }

    pub fn evaluate_advanced(&mut self) -> usize {
        let original_exp = self.exp.clone();
        while self.exp.len() > 1 {
            while let Some((opening_parenthesis, closing_parenthesis)) = self.deepest_parenthesis_index() {
                if closing_parenthesis - opening_parenthesis > 2 {
                    self.resolve_complex_first_from_index(opening_parenthesis + 1, closing_parenthesis - 1);
                } else {
                    self.exp.remove(closing_parenthesis);
                    self.exp.remove(opening_parenthesis);
                }
            }

            self.resolve_complex_first_from_index(0, self.exp.len() - 1);
        }

        let result = self.exp[0].parse().unwrap();
        self.exp = original_exp;
        result
    }

    pub fn resolve_complex_first_from_index(&mut self, start_index: usize, end_index: usize) {
        if let Some(addition_index) = self.exp[start_index..=end_index].iter().enumerate().find_map(|(i, e)| {
            if e.as_str() == "+" {
                Some(i)
            } else {
                None
            }
        }).and_then(|i| Some(i + start_index)) {
            let value = self.exp[addition_index - 1].parse::<usize>().unwrap() + self.exp[addition_index + 1].parse::<usize>().unwrap();
            self.exp.insert(addition_index - 1, value.to_string());
            self.exp.drain(addition_index..=addition_index + 2);
        } else {
            let value = self.exp[start_index].parse::<usize>().unwrap() * self.exp[start_index + 2].parse::<usize>().unwrap();
            self.exp.insert(start_index, value.to_string());
            self.exp.drain(start_index + 1..=start_index + 3);
        }
    }

    fn deepest_parenthesis_index(&self) -> Option<(usize, usize)> {
        if let Some(first_closing_parenthesis) = self.exp.iter().position(|i| i.as_str() == ")") {
            let opening_parenthesis = self.exp[..=first_closing_parenthesis].iter().enumerate().filter_map(|(i, e)| {
                if e.as_str() == "(" {
                    Some(i)
                } else {
                    None
                }
            }).max().unwrap();

            Some((opening_parenthesis, first_closing_parenthesis))
        } else {
            None
        }
    }
}

impl From<&str> for Expression {
    fn from(s: &str) -> Self {
        let mut exp = Vec::new();

        for i in s.split(" ") {
            let mut raw_value = i;

            while raw_value.starts_with("(") {
                exp.push(String::from("("));
                raw_value = &raw_value[1..];
            }

            exp.push(raw_value.replace(")", "").to_string());

            while raw_value.ends_with(")") {
                exp.push(String::from(")"));
                raw_value = &raw_value[..raw_value.len() - 1];
            }
        }

        Self {
            exp,
        }
    }
}
