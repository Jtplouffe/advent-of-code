use std::collections::VecDeque;

use crate::{hasher, lens::Lens, step::Step};

pub struct LensBoxes<'a> {
    boxes: Vec<VecDeque<Lens<'a>>>,
}

impl<'a> LensBoxes<'a> {
    pub fn new() -> Self {
        Self {
            boxes: vec![VecDeque::new(); 256],
        }
    }

    pub fn execute_steps(&mut self, steps: &[Step<'a>]) {
        'outer: for step in steps {
            match *step {
                Step::Remove(label) => {
                    let hash = hasher::hash(label);
                    let destination_box = &mut self.boxes[hash as usize];
                    for i in 0..destination_box.len() {
                        if destination_box[i].label == label {
                            destination_box.remove(i);
                            break;
                        }
                    }
                }
                Step::Upsert(label, focal_length) => {
                    let hash = hasher::hash(label);
                    let destination_box = &mut self.boxes[hash as usize];
                    for lens in destination_box.iter_mut() {
                        if lens.label == label {
                            lens.focal_length = focal_length;
                            continue 'outer;
                        }
                    }

                    destination_box.push_back(Lens {
                        label,
                        focal_length,
                    })
                }
            }
        }
    }

    pub fn focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_index, lens)| {
                lens.iter()
                    .enumerate()
                    .map(|(lens_index, lens)| {
                        lens.focal_length as u32 * (lens_index as u32 + 1) * (box_index as u32 + 1)
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}
