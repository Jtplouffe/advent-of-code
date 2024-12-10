#[derive(Clone)]
pub struct EmptyBlock {
    pub width: usize,
}

impl EmptyBlock {
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    pub fn shrink_by(&mut self, shrink_by: usize) {
        self.width -= shrink_by
    }
}
