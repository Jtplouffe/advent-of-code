use crate::pulse::Pulse;

pub struct FlipFlop<'a> {
    pub name: &'a str,
    pub destinations: Vec<&'a str>,
    state: bool,
}

impl<'a> FlipFlop<'a> {
    pub fn input(&mut self, pulse: Pulse) -> Option<Vec<(&'a str, Pulse)>> {
        if pulse == Pulse::High {
            return None;
        }

        self.state = !self.state;

        let pulse = if self.state { Pulse::High } else { Pulse::Low };

        Some(
            self.destinations
                .iter()
                .map(move |&destination| (destination, pulse))
                .collect(),
        )
    }
}

impl<'a> FlipFlop<'a> {
    pub fn new(name: &'a str, destinations: Vec<&'a str>) -> Self {
        Self {
            name,
            destinations,
            state: false,
        }
    }
}
