use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

pub struct RebootStep {
    pub on_off: bool,
    pub x_range: RangeInclusive<isize>,
    pub y_range: RangeInclusive<isize>,
    pub z_range: RangeInclusive<isize>,
}

impl From<&str> for RebootStep {
    fn from(s: &str) -> Self {
        let (on_off, ranges) = s.split_once(" ").unwrap();
        let on_off = if on_off == "on" { true } else { false };

        let mut range_split = ranges.split(",");

        let (x, y, z) = (
            range_split.next().unwrap(),
            range_split.next().unwrap(),
            range_split.next().unwrap(),
        );
        let (x, y, z) = (
            x.split("=").nth(1).unwrap(),
            y.split("=").nth(1).unwrap(),
            z.split("=").nth(1).unwrap(),
        );

        let (mut x_split, mut y_split, mut z_split) = (x.split(".."), y.split(".."), z.split(".."));
        let (x_range, y_range, z_range) = (
            isize::from_str(x_split.next().unwrap()).unwrap()
                ..=isize::from_str(x_split.next().unwrap()).unwrap(),
            isize::from_str(y_split.next().unwrap()).unwrap()
                ..=isize::from_str(y_split.next().unwrap()).unwrap(),
            isize::from_str(z_split.next().unwrap()).unwrap()
                ..=isize::from_str(z_split.next().unwrap()).unwrap(),
        );

        Self {
            on_off,
            x_range,
            y_range,
            z_range,
        }
    }
}
