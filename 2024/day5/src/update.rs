pub struct Update {
    pub pages: Vec<usize>,
}

impl Update {
    pub fn new(pages: Vec<usize>) -> Self {
        Self { pages }
    }

    pub fn middle_page(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        let pages = value.split(",").map(|page| page.parse().unwrap()).collect();

        Self { pages }
    }
}
