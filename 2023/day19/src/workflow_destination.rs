#[derive(Clone, Copy)]
pub enum WorkflowDestination<'a> {
    Workflow(&'a str),
    Accept,
    Reject,
}

impl<'a> From<&'a str> for WorkflowDestination<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            workflow => Self::Workflow(workflow),
        }
    }
}
