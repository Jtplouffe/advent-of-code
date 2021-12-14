pub enum Orientation {
    Vertical,
    Horizontal,
}

impl From<&str> for Orientation {
    fn from(o: &str) -> Self {
        match o {
            "y" => Self::Vertical,
            "x" => Self::Horizontal,
            _ => panic!("Unknown orientation: {}", o),
        }
    }
}
