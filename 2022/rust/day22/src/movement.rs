#[derive(Debug)]
pub enum Movement {
    Move(usize),
    Rotate(Rotation),
}

impl Movement {
    pub fn movements_from_str(s: &str) -> Vec<Self> {
        let mut movements = Vec::new();

        let mut number_accumulator = String::new();

        for c in s.trim().chars() {
            if c.is_ascii_digit() {
                number_accumulator.push(c);
            } else {
                if !number_accumulator.is_empty() {
                    movements.push(Self::Move(number_accumulator.parse().unwrap()));
                    number_accumulator.clear();
                }

                movements.push(Self::Rotate(Rotation::from(c)));
            }
        }

        if !number_accumulator.is_empty() {
            movements.push(Self::Move(number_accumulator.parse().unwrap()));
        }

        movements
    }
}

#[derive(Debug)]
pub enum Rotation {
    Left,
    Right,
}

impl From<char> for Rotation {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}
