use crate::direction::Direction;

pub struct DigPlanLine {
    pub direction: Direction,
    pub length: usize,
}

impl From<&str> for DigPlanLine {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        let (direction, length) = (split.next().unwrap(), split.next().unwrap());

        Self {
            direction: direction.chars().next().unwrap().into(),
            length: length.parse().unwrap(),
        }
    }
}
