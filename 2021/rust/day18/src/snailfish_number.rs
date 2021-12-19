use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum SnailfishNumber {
    Value(u32),
    Pair(Box<(SnailfishNumber, SnailfishNumber)>),
}

impl SnailfishNumber {
    pub fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    pub fn explode(&mut self) -> bool {
        if let Some((pos, left_value, right_value)) = self.find_explode(0, &mut 0) {
            if pos > 0 {
                self.add_to(pos - 1, left_value, &mut 0);
            }
            self.add_to(pos + 1, right_value, &mut 0);
            true
        } else {
            false
        }
    }

    fn find_explode(&mut self, depth: usize, current_pos: &mut usize) -> Option<(usize, u32, u32)> {
        match self {
            Self::Value(_) => {
                *current_pos += 1;
                None
            }
            Self::Pair(pair) => {
                let (left, right) = &mut **pair;

                if left.is_value() && right.is_value() && depth == 4 {
                    let (left_value, right_value) = (left.value(), right.value());

                    *self = Self::Value(0);
                    Some((*current_pos, left_value, right_value))
                } else {
                    if let Some(result) = left.find_explode(depth + 1, current_pos) {
                        Some(result)
                    } else {
                        right.find_explode(depth + 1, current_pos)
                    }
                }
            }
        }
    }

    fn add_to(&mut self, pos: usize, value: u32, current_pos: &mut usize) {
        match self {
            Self::Value(v) => {
                if pos == *current_pos {
                    *self = Self::Value(*v + value);
                }
                *current_pos += 1;
            }
            Self::Pair(pair) => {
                let (left, right) = &mut **pair;

                left.add_to(pos, value, current_pos);
                right.add_to(pos, value, current_pos);
            }
        }
    }

    pub fn split(&mut self) -> bool {
        match self {
            Self::Value(v) => {
                if *v >= 10 {
                    let left_value = *v / 2;
                    let right_value = *v - left_value;
                    *self = Self::Pair(Box::new((Self::Value(left_value), Self::Value(right_value))));
                    true
                } else {
                    false
                }
            }
            Self::Pair(pair) => {
                let (left, right) = &mut **pair;
                left.split() || right.split()
            }
        }
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn value(&self) -> u32 {
        match self {
            Self::Value(v) => *v,
            _ => panic!("value called on a non Value variant"),
        }
    }

    pub fn get_magnitude(&self) -> u32 {
        match self {
            Self::Value(v) => *v,
            Self::Pair(pair) => {
                let (left, right) = &**pair;
                left.get_magnitude() * 3 + right.get_magnitude() * 2
            }
        }
    }
}

impl From<&str> for SnailfishNumber {
    fn from(s: &str) -> Self {
        if s.contains(",") {
            let s = &s[1..(s.len() - 1)];

            let (left, right) = split_at_top_level_comma(s);
            Self::Pair(Box::new((Self::from(left), Self::from(right))))
        } else {
            Self::Value(s.parse().unwrap())
        }
    }
}

fn split_at_top_level_comma(s: &str) -> (&str, &str) {
    let mut bracket_depth = 0;
    for (index, char) in s.chars().enumerate() {
        match char {
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            ',' => {
                if bracket_depth == 0 {
                    let (p1, p2) = s.split_at(index);
                    return (p1, &p2[1..]);
                }
            }
            _ => {}
        };
    }

    unreachable!()
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Self::Pair(Box::new((self, rhs)));
        sum.reduce();
        sum
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Pair(pair) => {
                let (left, right) = &**pair;

                write!(f, "[{},{}]", left, right)
            }
        }
    }
}