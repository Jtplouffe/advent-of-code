use crate::pulse::Pulse;

use self::{broadcast::Broadcast, conjunction::Conjunction, flip_flop::FlipFlop};

mod broadcast;
mod conjunction;
mod flip_flop;

pub const BROADCAST_NAME: &str = "broadcaster";

pub enum Module<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcast(Broadcast<'a>),
}

impl<'a> Module<'a> {
    pub fn is_conjunction(&self) -> bool {
        matches!(self, Self::Conjunction(_))
    }

    pub fn name(&self) -> &'a str {
        match self {
            Self::FlipFlop(flip_flop) => flip_flop.name,
            Self::Conjunction(conjunction) => conjunction.name,
            Self::Broadcast(_) => BROADCAST_NAME,
        }
    }

    pub fn has_destination(&self, name: &'a str) -> bool {
        let destinations = match self {
            Self::FlipFlop(flip_flop) => &flip_flop.destinations,
            Self::Conjunction(conjunction) => &conjunction.destinations,
            Self::Broadcast(broadcast) => &broadcast.destinations,
        };

        destinations.contains(&name)
    }

    pub fn input(&mut self, from: &'a str, pulse: Pulse) -> Option<Vec<(&'a str, Pulse)>> {
        match self {
            Self::FlipFlop(flip_flop) => flip_flop.input(pulse),
            Self::Conjunction(conjunction) => Some(conjunction.input(from, pulse)),
            Self::Broadcast(broadcast) => Some(broadcast.input(pulse)),
        }
    }
}

impl<'a> From<&'a str> for Module<'a> {
    fn from(value: &'a str) -> Self {
        let (module, destinations) = value.split_once(" -> ").unwrap();
        let destinations: Vec<_> = destinations.split(", ").collect();

        if module == BROADCAST_NAME {
            Self::Broadcast(Broadcast::new(destinations))
        } else if let Some(name) = module.strip_prefix('%') {
            Self::FlipFlop(FlipFlop::new(name, destinations))
        } else if let Some(name) = module.strip_prefix('&') {
            Self::Conjunction(Conjunction::new(name, destinations))
        } else {
            unreachable!()
        }
    }
}
