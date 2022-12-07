pub struct ListCommand {
    pub output: String,
}

impl ListCommand {
    pub fn new(output: String) -> Self {
        Self { output }
    }
}