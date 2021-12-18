use crate::area::Area;
use crate::probe::Probe;

pub struct ProbeLauncher {
    target_area: Area,
}

impl ProbeLauncher {
    pub fn new(target_area: Area) -> Self {
        Self { target_area }
    }

    fn min_starting_x(&self) -> isize {
        // Inverse nth triangle
        (self.target_area.min_x() as f32 * 2f32).sqrt().floor() as isize
    }

    pub fn highest_possible_height(&self) -> isize {
        -self.target_area.min_y() * (-self.target_area.min_y() - 1) / 2
    }

    pub fn target_hit_velocity_count(&self) -> isize {
        let mut velocity_count = 0;

        for x in self.min_starting_x()..=self.target_area.max_x() {
            for y in 0..1000 {
                let mut probe = Probe::new(x, y - 500, self.target_area);
                probe.run();
                if probe.is_in_target_area() {
                    velocity_count += 1;
                }
            }
        }

        velocity_count
    }
}
