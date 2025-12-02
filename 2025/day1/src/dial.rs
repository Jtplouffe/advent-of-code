use crate::rotation::Rotation;

pub struct Dial {
    rotations: Vec<Rotation>,
}

impl Dial {
    pub fn new(rotations: Vec<Rotation>) -> Self {
        Self { rotations }
    }

    pub fn count_zero_occurrences_at_end(&self) -> usize {
        let mut occurrences = 0;

        let mut current_position = 50;
        for rotation in &self.rotations {
            match *rotation {
                Rotation::Left(count) => current_position -= count as isize,
                Rotation::Right(count) => current_position += count as isize,
            }

            current_position = current_position.rem_euclid(100);

            if current_position == 0 {
                occurrences += 1;
            }
        }

        occurrences
    }

    pub fn count_zero_occurrences_anytime(&self) -> usize {
        let mut occurrences = 0;

        let mut current_position = 50;

        for rotation in &self.rotations {
            let new_position = match *rotation {
                Rotation::Left(count) => {
                    let new_position = current_position - count as isize;

                    if new_position <= 0 && current_position > 0 {
                        occurrences += 1;
                    }

                    if new_position < 0 {
                        occurrences += new_position.unsigned_abs() / 100;
                    }

                    new_position
                }
                Rotation::Right(count) => {
                    let new_position = current_position + count as isize;
                    if new_position > 99 {
                        occurrences += new_position as usize / 100
                    }
                    new_position
                }
            };

            current_position = new_position.rem_euclid(100);
        }

        occurrences
    }
}
