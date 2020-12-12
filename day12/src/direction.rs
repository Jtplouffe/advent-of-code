pub enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

impl Direction {
    pub fn value(&self) -> u8 {
        match self {
            Self::Right => 0,
            Self::Bottom => 1,
            Self::Left => 2,
            Self::Top => 3,
        }
    }

    pub fn rotate(&self, angle: i16) -> Self {
        let mut new_angle = self.value() as i8 + (angle / 90) as i8;

        while new_angle < 0 {
            new_angle += 4;
        }
        new_angle %= 4;

        Self::from(new_angle as u8)
    }
}

impl From<u8> for Direction {
    fn from(s: u8) -> Self {
        match s {
            0 => Self::Right,
            1 => Self::Bottom,
            2 => Self::Left,
            3 => Self::Top,
            _ => unreachable!(),
        }
    }
}