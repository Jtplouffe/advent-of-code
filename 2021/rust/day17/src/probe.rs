use crate::area::Area;

pub struct Probe {
    pub position: (isize, isize),
    pub velocity: (isize, isize),
    target_area: Area,
}

impl Probe {
    pub fn new(velocity_x: isize, velocity_y: isize, target_area: Area) -> Self {
        Self {
            position: (0, 0),
            velocity: (velocity_x, velocity_y),
            target_area,
        }
    }

    pub fn step(&mut self) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        self.velocity = ((self.velocity.0 - 1).max(0), self.velocity.1 - 1);
    }

    pub fn run(&mut self) {
        while !self.is_in_target_area() && !self.has_past_target_area() {
            self.step();
        }
    }

    pub fn is_in_target_area(&self) -> bool {
        self.target_area
            .is_in_area(self.position.0, self.position.1)
    }

    pub fn has_past_target_area(&self) -> bool {
        self.target_area.max_x() < self.position.0 || self.position.1 < self.target_area.min_y()
    }
}
