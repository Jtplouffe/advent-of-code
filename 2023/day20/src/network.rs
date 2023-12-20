use std::collections::HashMap;

use crate::{
    modules::{Module, BROADCAST_NAME},
    pulse::Pulse,
};

pub struct Network<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Network<'a> {
    pub fn button_push_signal_count_product(&mut self, push_count: usize) -> usize {
        let (mut high_signal_count, mut low_signal_count) = (0, 0);

        for _ in 0..push_count {
            let mut current_tick_inputs = vec![("BUTTON", BROADCAST_NAME, Pulse::Low)];
            let mut next_tick_inputs = Vec::new();

            while !current_tick_inputs.is_empty() {
                for &(from_module, to_module, pulse) in &current_tick_inputs {
                    match pulse {
                        Pulse::High => high_signal_count += 1,
                        Pulse::Low => low_signal_count += 1,
                    };

                    if let Some(module) = self.modules.get_mut(to_module) {
                        if let Some(next_inputs) = module.input(from_module, pulse) {
                            next_tick_inputs.extend(
                                next_inputs
                                    .iter()
                                    .map(|&(destination, pulse)| (to_module, destination, pulse)),
                            );
                        }
                    }
                }

                std::mem::swap(&mut current_tick_inputs, &mut next_tick_inputs);
                next_tick_inputs.clear();
            }
        }

        high_signal_count * low_signal_count
    }
}

impl<'a> From<&'a str> for Network<'a> {
    fn from(value: &'a str) -> Self {
        let mut modules: HashMap<_, _> = value
            .lines()
            .map(|line| {
                let module = Module::from(line);
                (module.name(), module)
            })
            .collect();

        let conjunction_inputs: Vec<_> = modules
            .iter()
            .filter_map(|(name, module)| {
                if !module.is_conjunction() {
                    return None;
                }

                let inputs: Vec<_> = modules
                    .iter()
                    .filter_map(|(n, m)| {
                        if m.has_destination(name) {
                            Some(*n)
                        } else {
                            None
                        }
                    })
                    .collect();
                Some((*name, inputs))
            })
            .collect();

        for (conjunction_name, inputs) in conjunction_inputs {
            match modules.get_mut(conjunction_name) {
                Some(Module::Conjunction(conjunction)) => {
                    conjunction.register_inputs(&inputs);
                }
                _ => unreachable!(),
            }
        }

        Self { modules }
    }
}
