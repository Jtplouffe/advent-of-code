pub struct ChangeDirectoryCommand<'a> {
    pub directory: &'a str,
}

impl<'a> ChangeDirectoryCommand<'a> {
    pub fn new(directory: &'a str) -> Self {
        return Self { directory }
    }
}