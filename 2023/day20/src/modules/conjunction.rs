use std::collections::HashMap;

use crate::pulse::Pulse;

pub struct Conjunction<'a> {
    pub name: &'a str,
    pub input_pulses: HashMap<&'a str, Pulse>,
    pub destinations: Vec<&'a str>,
}

impl<'a> Conjunction<'a> {
    pub fn register_inputs(&mut self, inputs: &[&'a str]) {
        if !self.input_pulses.is_empty() {
            self.input_pulses.clear();
        }

        for input in inputs {
            self.input_pulses.insert(input, Pulse::Low);
        }
    }

    pub fn input(&mut self, from: &'a str, pulse: Pulse) -> Vec<(&'a str, Pulse)> {
        self.input_pulses.entry(from).and_modify(|p| *p = pulse);

        let all_inputs_are_high_pulse = self.input_pulses.values().all(Pulse::is_high);

        let pulse = if all_inputs_are_high_pulse {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.destinations
            .iter()
            .map(|&destination| (destination, pulse))
            .collect()
    }
}

impl<'a> Conjunction<'a> {
    pub fn new(name: &'a str, destinations: Vec<&'a str>) -> Self {
        Self {
            name,
            input_pulses: HashMap::new(),
            destinations,
        }
    }
}
