pub struct Octopus {
    initial_energy: u8,
    energy: u8,
}

impl Octopus {
    // Return true if it flashes surrounding octopus
    pub fn energize(&mut self) -> bool {
        if self.has_flashed() {
            // Already flashed
            return false;
        }

        self.energy += 1;

        return self.energy > 9;
    }

    pub fn has_flashed(&self) -> bool {
        self.energy > 9
    }

    pub fn next_step(&mut self) {
        if self.has_flashed() {
            self.energy = 0;
        }
    }

    pub fn reset(&mut self) {
        self.energy = self.initial_energy;
    }
}

impl Octopus {
    pub fn new(initial_energy: u8) -> Self {
        Self {
            initial_energy,
            energy: initial_energy,
        }
    }
}
