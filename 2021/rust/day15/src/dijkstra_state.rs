use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DijkstraState {
    pub distance: usize,
    pub index: usize,
}

impl DijkstraState {
    pub fn new(distance: usize, index: usize) -> Self {
        Self { distance, index }
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance).then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
