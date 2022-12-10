use std::collections::HashSet;

use crate::step::{Direction, Step};

type Position = (isize, isize);

pub struct RopeGrid {
    head_position: Position,
    knots: Vec<Position>,
    tail_positions: HashSet<isize>,
}

impl RopeGrid {
    pub fn new(knot_count: usize) -> Self {
        assert!(knot_count >= 1);

        let mut instance = Self {
            head_position: (0, 0),
            knots: vec![(0, 0); knot_count],
            tail_positions: HashSet::new(),
        };

        instance.save_tail_position();
        instance
    }

    pub fn navigate(&mut self, steps: &[Step]) -> usize {
        for step in steps {
            self.apply_step(step);
        }

        self.tail_positions.len()
    }

    fn apply_step(&mut self, step: &Step) {
        for _ in 0..step.count {
            match step.direction {
                Direction::Left => self.head_position.0 -= 1,
                Direction::Up => self.head_position.1 -= 1,
                Direction::Right => self.head_position.0 += 1,
                Direction::Down => self.head_position.1 += 1,
            }

            for knot_index in 0..self.knots.len() {
                self.move_knot(knot_index);
            }
        }
    }

    fn move_knot(&mut self, knot_index: usize) {
        let knot = self.knots[knot_index];
        let parent = if knot_index == 0 {
            self.head_position
        } else {
            self.knots[knot_index - 1]
        };

        let delta_x = parent.0 - knot.0;
        let delta_y = parent.1 - knot.1;
        if delta_x.abs() <= 1 && delta_y.abs() <= 1 {
            return;
        }

        if delta_x.abs() >= 2 {
            self.knots[knot_index].0 += delta_x - delta_x.signum();

            if delta_y.abs() == 1 {
                self.knots[knot_index].1 += delta_y.signum();
            }
        }

        if delta_y.abs() >= 2 {
            self.knots[knot_index].1 += delta_y - delta_y.signum();

            if delta_x.abs() == 1 {
                self.knots[knot_index].0 += delta_x.signum();
            }
        }

        self.save_tail_position()
    }

    fn save_tail_position(&mut self) {
        self.tail_positions
            .insert(signed_cantor_pair(*self.knots.last().unwrap()));
    }
}

// Signed version of cantor pairing
// https://www.vertexfragment.com/ramblings/cantor-szudzik-pairing-functions/
fn signed_cantor_pair((mut n1, mut n2): (isize, isize)) -> isize {
    if n1 >= 0 {
        n1 *= 2;
    } else {
        n1 = (-2 * n1) - 1;
    }

    if n2 >= 0 {
        n2 *= 2;
    } else {
        n2 = (-2 * n2) - 1;
    }

    (n1 + n2) * (n1 + n2 + 1) / 2 + n2
}
