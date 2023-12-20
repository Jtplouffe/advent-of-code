use crate::pulse::Pulse;

pub struct Broadcast<'a> {
    pub destinations: Vec<&'a str>,
}

impl<'a> Broadcast<'a> {
    pub fn input(&mut self, pulse: Pulse) -> Vec<(&'a str, Pulse)> {
        self.destinations
            .iter()
            .map(|&destination| (destination, pulse))
            .collect()
    }
}

impl<'a> Broadcast<'a> {
    pub fn new(destinations: Vec<&'a str>) -> Self {
        Self { destinations }
    }
}
