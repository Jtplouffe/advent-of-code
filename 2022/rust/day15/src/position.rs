#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let s = s.replace("x=", "").replace("y=", "");
        let (x, y) = s.split_once(", ").unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}
