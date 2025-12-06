use std::ops::{Add, Mul};

pub enum Operator {
    Addition,
    Multiplication,
}

impl Operator {
    pub fn calculate<T>(&self, left: T, right: T) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        match self {
            Self::Addition => left + right,
            Self::Multiplication => left * right,
        }
    }
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Addition,
            '*' => Self::Multiplication,
            _ => unreachable!(),
        }
    }
}
