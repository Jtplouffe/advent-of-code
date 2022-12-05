pub struct Action {
    pub move_from_index: usize,
    pub move_to_index: usize,
    pub move_count: usize,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let mut split = s.split(' ');
        let (_, move_count, _, move_from_index, _, move_to_index) = (
            split.next(),
            split.next().unwrap().parse::<usize>().unwrap(),
            split.next(),
            split.next().unwrap().parse::<usize>().unwrap() - 1,
            split.next(),
            split.next().unwrap().parse::<usize>().unwrap() - 1,
        );

        Self {
            move_from_index,
            move_to_index,
            move_count,
        }
    }
}
