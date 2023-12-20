#[derive(Clone, Copy, PartialEq)]
pub enum Pulse {
    High,
    Low,
}

impl Pulse {
    pub fn is_high(&self) -> bool {
        matches!(self, Self::High)
    }
}
