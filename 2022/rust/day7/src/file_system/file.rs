#[derive(Debug)]
pub struct File<'a> {
    pub name: &'a str,
    pub size: usize,
}

impl<'a> File<'a> {
    pub fn new(name: &'a str, size: usize) -> Self {
        Self { name, size }
    }
}