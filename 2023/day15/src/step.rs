pub enum Step<'a> {
    Remove(&'a str),
    Upsert(&'a str, u8),
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(value: &'a str) -> Self {
        if let Some(label) = value.strip_suffix('-') {
            Self::Remove(label)
        } else {
            let (label, focal_length) = value.split_once('=').unwrap();
            Self::Upsert(label, focal_length.parse().unwrap())
        }
    }
}
