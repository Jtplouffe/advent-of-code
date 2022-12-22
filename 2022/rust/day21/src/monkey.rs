#[derive(Clone)]
pub struct Monkey<'a> {
    pub name: &'a str,
    pub job: MonkeyJob<'a>,
}

impl<'a> From<&'a str> for Monkey<'a> {
    fn from(s: &'a str) -> Self {
        let (name, job) = s.split_once(": ").unwrap();

        Self {
            name,
            job: job.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MonkeyJob<'a> {
    Yell(i64),
    Arithmetic((&'a str, Operation, &'a str)),
}

impl<'a> From<&'a str> for MonkeyJob<'a> {
    fn from(s: &'a str) -> Self {
        if s.contains(' ') {
            let mut split = s.split(" ");
            let first_monkey_name = split.next().unwrap();
            let operation = split.next().unwrap().into();
            let second_monkey_name = split.next().unwrap();

            Self::Arithmetic((first_monkey_name, operation, second_monkey_name))
        } else {
            Self::Yell(s.parse().unwrap())
        }
    }
}

#[derive(Clone, Copy)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    pub fn solve(&self, left_value: i64, right_value: i64) -> i64 {
        match self {
            Self::Addition => left_value + right_value,
            Self::Subtraction => left_value - right_value,
            Self::Multiplication => left_value * right_value,
            Self::Division => left_value / right_value,
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Addition,
            "-" => Self::Subtraction,
            "*" => Self::Multiplication,
            "/" => Self::Division,
            _ => unreachable!(),
        }
    }
}
