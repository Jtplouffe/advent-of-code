use std::collections::HashSet;

use crate::reboot_step::RebootStep;

pub struct Engine {
    reboot_steps: Vec<RebootStep>,
    on_cubes: HashSet<(isize, isize, isize)>,
}

impl Engine {
    pub fn reboot(&mut self) {
        for reboot_step in &self.reboot_steps {
            for z in reboot_step.z_range.clone() {
                if z < -50 || z > 50 {
                    continue;
                }

                for y in reboot_step.y_range.clone() {
                    if y < -50 || y > 50 {
                        continue;
                    }

                    for x in reboot_step.x_range.clone() {
                        if x < -50 || x > 50 {
                            continue;
                        }

                        if reboot_step.on_off {
                            self.on_cubes.insert((x, y, z));
                        } else {
                            self.on_cubes.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }

    pub fn on_cube_count(&self) -> usize {
        self.on_cubes.len()
    }
}

impl From<&str> for Engine {
    fn from(s: &str) -> Self {
        let reboot_steps: Vec<_> = s
            .lines()
            .map(RebootStep::from)
            .collect();

        Self {
            reboot_steps,
            on_cubes: HashSet::new(),
        }
    }
}
