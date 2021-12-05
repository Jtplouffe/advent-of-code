use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();

        Self {
            x: i32::from_str(x).unwrap(),
            y: i32::from_str(y).unwrap(),
        }
    }
}
