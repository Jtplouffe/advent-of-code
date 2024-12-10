#[derive(Clone)]
pub struct File {
    pub id: usize,
    pub width: usize,
}

impl File {
    pub fn new(id: usize, width: usize) -> Self {
        Self { id, width }
    }

    pub fn split(&self, size: usize) -> (Self, Option<Self>) {
        assert!(size <= self.width);

        let split = Self::new(self.id, size);
        let remaining = match self.width - size {
            0 => None,
            width => Some(Self::new(self.id, width)),
        };

        (split, remaining)
    }
}
