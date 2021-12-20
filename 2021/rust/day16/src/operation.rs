#[derive(Debug)]
pub enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operation {
    pub fn new(type_id: u8) -> Self {
        match type_id {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Minimum,
            3 => Operation::Maximum,
            5 => Operation::GreaterThan,
            6 => Operation::LessThan,
            7 => Operation::EqualTo,
            _ => panic!("Unknown type id: {}", type_id),
        }
    }
}
