use std::str::FromStr;

use crate::octopus::Octopus;

pub struct OctopusGrid {
    octopuses: Vec<Vec<Octopus>>,
}

impl OctopusGrid {
    pub fn flash_count_after_steps(&mut self, steps: usize) -> u32 {
        let mut flash_count = 0;

        for _ in 0..steps {
            self.do_step();

            for x in 0..10 {
                for y in 0..10 {
                    if self.octopuses[y][x].has_flashed() {
                        flash_count += 1;
                    }

                    self.octopuses[y][x].next_step();
                }
            }
        }

        flash_count
    }

    pub fn first_step_with_synchronized_flash(&mut self) -> usize {
        let mut i = 0;
        loop {
            self.do_step();
            i += 1;

            if self.octopuses.iter().flat_map(|l| l).all(Octopus::has_flashed) {
                return i;
            }

            self.octopuses.iter_mut().flat_map(|l| l).for_each(Octopus::next_step);
        }
    }

    fn do_step(&mut self) {
        for x in 0..10 {
            for y in 0..10 {
                if self.octopuses[y][x].energize() {
                    self.octopus_flashed(x, y);
                }
            }
        }
    }

    fn octopus_flashed(&mut self, x: usize, y: usize) {
        for (x, y) in self.get_indexes_around(x, y) {
            if self.octopuses[y][x].energize() {
                self.octopus_flashed(x, y);
            }
        }
    }

    fn get_indexes_around(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut indexes = Vec::new();

        let x = x as isize;
        let y = y as isize;

        for x in (x - 1).max(0)..=(x + 1).min(9) {
            for y in (y - 1).max(0)..=(y + 1).min(9) {
                indexes.push((x as usize, y as usize));
            }
        }

        indexes
    }

    pub fn reset(&mut self) {
        for o in self.octopuses.iter_mut().flat_map(|l| l) {
            o.reset();
        }
    }
}

impl From<&str> for OctopusGrid {
    fn from(s: &str) -> Self {
        let octopuses = s
            .lines()
            .map(|l| {
                l.split("")
                    .filter_map(|o| {
                        if let Ok(energy) = u8::from_str(o) {
                            Some(Octopus::new(energy))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        Self { octopuses }
    }
}
