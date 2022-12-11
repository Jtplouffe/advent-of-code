use crate::monkey::Monkey;

pub struct KeepAway {
    pub monkeys: Vec<Monkey>,
    divide_worry_level_by_after_inspection: usize,
    monkey_test_divisible_by_product: usize,
}

impl KeepAway {
    pub fn new(input: &str, divide_worry_level_by_after_inspection: usize) -> Self {
        let monkeys: Vec<_> = input.split("\n\n").map(Monkey::from).collect();
        let monkey_test_divisible_by_product = monkeys
            .iter()
            .map(|monkey| monkey.test_divisible_by)
            .product();
        Self {
            monkeys,
            divide_worry_level_by_after_inspection,
            monkey_test_divisible_by_product,
        }
    }

    pub fn execute_rounds(&mut self, round_count: usize) {
        for _ in 0..round_count {
            for monkey_index in 0..self.monkeys.len() {
                self.execute_monkey_turn(monkey_index);
            }
        }
    }

    fn execute_monkey_turn(&mut self, monkey_index: usize) {
        while !self.monkeys[monkey_index].items.is_empty() {
            self.monkeys[monkey_index].inspection_count += 1;

            let mut item = self.monkeys[monkey_index].items.remove(0);
            item = self.monkeys[monkey_index].operation.execute(item)
                % self.monkey_test_divisible_by_product;

            if self.divide_worry_level_by_after_inspection != 0 {
                item /= self.divide_worry_level_by_after_inspection;
            }

            let send_item_to_monkey_index =
                if item % self.monkeys[monkey_index].test_divisible_by == 0 {
                    self.monkeys[monkey_index].if_test_true_throw_to_monkey
                } else {
                    self.monkeys[monkey_index].if_test_false_throw_to_monkey
                };

            self.monkeys[send_item_to_monkey_index].items.push(item);
        }
    }

    pub fn monkey_business(&self) -> usize {
        let mut inspection_count = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspection_count)
            .collect::<Vec<_>>();
        inspection_count.sort_by(|a, b| b.cmp(a));

        inspection_count[0] * inspection_count[1]
    }
}
