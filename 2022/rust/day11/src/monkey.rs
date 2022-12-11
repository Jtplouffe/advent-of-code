pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: Operation,
    pub test_divisible_by: usize,
    pub if_test_true_throw_to_monkey: usize,
    pub if_test_false_throw_to_monkey: usize,
    pub inspection_count: usize,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        lines.next(); // Ignore the first line

        let (_, starting_items) = lines.next().unwrap().split_once(": ").unwrap();
        let starting_items = starting_items
            .split(", ")
            .map(|starting_item| starting_item.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (_, operation) = lines.next().unwrap().split_once("= ").unwrap();
        let operation = Operation::from(operation);

        let (_, test_divisible_by) = lines.next().unwrap().split_once("y ").unwrap();
        let test_divisible_by = test_divisible_by.parse().unwrap();

        let (_, if_test_true_throw_to_monkey) = lines.next().unwrap().split_once("y ").unwrap();
        let if_test_true_throw_to_monkey = if_test_true_throw_to_monkey.parse().unwrap();

        let (_, if_test_false_throw_to_monkey) = lines.next().unwrap().split_once("y ").unwrap();
        let if_test_false_throw_to_monkey = if_test_false_throw_to_monkey.parse().unwrap();

        Self {
            items: starting_items,
            operation,
            test_divisible_by,
            if_test_true_throw_to_monkey,
            if_test_false_throw_to_monkey,
            inspection_count: 0,
        }
    }
}

pub struct Operation {
    pub left_value: OperationValue,
    pub symbol: OperationSymbol,
    pub right_value: OperationValue,
}

impl Operation {
    pub fn execute(&self, old_value: usize) -> usize {
        let left_value = match self.left_value {
            OperationValue::OldValue => old_value,
            OperationValue::Constant(value) => value,
        };

        let right_value = match self.right_value {
            OperationValue::OldValue => old_value,
            OperationValue::Constant(value) => value,
        };

        match self.symbol {
            OperationSymbol::Addition => left_value + right_value,
            OperationSymbol::Multiplication => left_value * right_value,
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let mut split = s.split(" ");
        let left_value = split.next().unwrap().into();
        let symbol = split.next().unwrap().into();
        let right_value = split.next().unwrap().into();

        Self {
            left_value,
            symbol,
            right_value,
        }
    }
}

pub enum OperationValue {
    OldValue,
    Constant(usize),
}

impl From<&str> for OperationValue {
    fn from(s: &str) -> Self {
        match s {
            "old" => Self::OldValue,
            constant => Self::Constant(constant.parse().unwrap()),
        }
    }
}

pub enum OperationSymbol {
    Addition,
    Multiplication,
}

impl From<&str> for OperationSymbol {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Addition,
            "*" => Self::Multiplication,
            _ => unreachable!(),
        }
    }
}
