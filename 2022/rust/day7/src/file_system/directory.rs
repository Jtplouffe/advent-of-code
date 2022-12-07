use crate::file_system::file::File;

#[derive(Debug)]
pub struct Directory<'a> {
    pub name: &'a str,
    files: Vec<File<'a>>
}

impl<'a> Directory<'a> {
    pub fn new(name: &'a str, files: Vec<File<'a>>) -> Self {
        Self { name, files }
    }

    pub fn total_size(&self) -> usize {
        self.files.iter().map(|file| file.size).sum()
    }
}
