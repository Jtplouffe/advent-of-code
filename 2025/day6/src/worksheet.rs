use crate::operator::Operator;

pub struct Worksheet {
    operands: Vec<Vec<usize>>,
    operators: Vec<Operator>,
}

impl Worksheet {
    pub fn new_horizontal_operands(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();

        let operators: Vec<_> = lines
            .last()
            .unwrap()
            .chars()
            .filter(|char| !char.is_whitespace())
            .map(Operator::from)
            .collect();

        let mut operands = vec![vec![]; operators.len()];
        for line in &lines[..lines.len() - 1] {
            let mut operation_index = 0;
            for chunk in line.split(' ') {
                if chunk.is_empty() {
                    continue;
                }

                let operand = chunk.parse().unwrap();
                operands[operation_index].push(operand);
                operation_index += 1;
            }
        }

        Self {
            operands,
            operators,
        }
    }

    pub fn new_vertical_operands(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();

        let operator_indexes: Vec<_> = lines
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .filter_map(
                |(i, char)| {
                    if char.is_whitespace() { None } else { Some(i) }
                },
            )
            .collect();

        let mut operands = Vec::<Vec<usize>>::with_capacity(operator_indexes.len());

        for operand_index in operator_indexes {
            let mut current_operands = Vec::new();

            for char_index in operand_index.. {
                let mut current_operand = String::new();

                for line in &lines[..lines.len() - 1] {
                    let digit = line.chars().nth(char_index).unwrap_or(' ');
                    if !digit.is_whitespace() {
                        current_operand.push(digit);
                    }
                }

                if current_operand.is_empty() {
                    break;
                }

                current_operands.push(current_operand);
            }

            let current_operands = current_operands
                .iter()
                .map(|operand| operand.parse().unwrap())
                .collect();
            operands.push(current_operands);
        }

        let operators = lines
            .last()
            .unwrap()
            .chars()
            .filter(|char| !char.is_whitespace())
            .map(Operator::from)
            .collect();

        Self {
            operands,
            operators,
        }
    }

    pub fn grand_total(&self) -> usize {
        let mut results = Vec::with_capacity(self.operators.len());

        for (i, operator) in self.operators.iter().enumerate() {
            let total = self.operands[i]
                .iter()
                .copied()
                .reduce(|total, current| operator.calculate(total, current))
                .unwrap();
            results.push(total);
        }

        results.iter().sum()
    }
}
