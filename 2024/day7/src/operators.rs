#[derive(Clone, Copy)]
pub enum Operators {
    Addition,
    Multiplication,
    Concatenation,
}

impl Operators {
    pub fn evaluate(&self, left: u64, right: u64) -> u64 {
        match self {
            Operators::Addition => left + right,
            Operators::Multiplication => left * right,
            Operators::Concatenation => (left.to_string() + &right.to_string()).parse().unwrap(),
        }
    }
}
