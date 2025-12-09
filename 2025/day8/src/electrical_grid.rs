use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use crate::position::Position;

pub struct ElectricalGrid {
    junction_boxes: Vec<Position>,
}

impl ElectricalGrid {
    pub fn three_largest_circuit_size_product(&self, max_connection_count: usize) -> usize {
        let connection_count = max_connection_count.min(self.junction_boxes.len());

        let junction_box_combinations = self.sorted_junction_box_combinations();

        // Most likely more capacity than needed, but at least we won't have to re-allocate, and entries are small.
        // Used to keep track of the circuit of a specific junction box
        let mut junction_box_circuit =
            HashMap::<&Position, usize>::with_capacity(connection_count * 2);
        // Used to keep track of all the junction boxes of a specific circuit.
        let mut circuit_junction_boxes = HashMap::<usize, HashSet<&Position>>::new();
        let mut next_circuit_id = 0;

        for (source_junction_box, destination_junction_box) in
            &junction_box_combinations[..connection_count]
        {
            self.handle_junction_boxes_connection(
                &mut junction_box_circuit,
                &mut circuit_junction_boxes,
                next_circuit_id,
                source_junction_box,
                destination_junction_box,
            );

            if circuit_junction_boxes.contains_key(&next_circuit_id) {
                next_circuit_id += 1;
            }
        }

        let mut circuits: Vec<_> = circuit_junction_boxes.into_values().collect();
        circuits.sort_by_key(|circuit| Reverse(circuit.len()));

        circuits[..3].iter().map(|circuit| circuit.len()).product()
    }

    pub fn single_circuit_last_connection_x_product(&self) -> isize {
        let junction_box_combinations = self.sorted_junction_box_combinations();

        // Most likely more capacity than needed, but at least we won't have to re-allocate, and entries are small.
        // Used to keep track of the circuit of a specific junction box
        let mut junction_box_circuit =
            HashMap::<&Position, usize>::with_capacity(self.junction_boxes.len() * 2);
        // Used to keep track of all the junction boxes of a specific circuit.
        let mut circuit_junction_boxes = HashMap::<usize, HashSet<&Position>>::new();
        let mut next_circuit_id = 0;

        for (source_junction_box, destination_junction_box) in &junction_box_combinations {
            self.handle_junction_boxes_connection(
                &mut junction_box_circuit,
                &mut circuit_junction_boxes,
                next_circuit_id,
                source_junction_box,
                destination_junction_box,
            );

            if circuit_junction_boxes.contains_key(&next_circuit_id) {
                next_circuit_id += 1;
            }

            // If every junction box is in a circuit and only a single circuit exists
            if junction_box_circuit.len() == self.junction_boxes.len()
                && circuit_junction_boxes.len() == 1
            {
                return source_junction_box.x * destination_junction_box.x;
            }
        }

        panic!("Could not form a single circuit");
    }

    fn handle_junction_boxes_connection<'a>(
        &self,
        junction_box_circuit: &mut HashMap<&'a Position, usize>,
        circuit_junction_boxes: &mut HashMap<usize, HashSet<&'a Position>>,
        next_circuit_id: usize,
        source_junction_box: &'a Position,
        destination_junction_box: &'a Position,
    ) {
        let source_circuit_id = junction_box_circuit.get(source_junction_box).copied();
        let destination_circuit_id = junction_box_circuit.get(destination_junction_box).copied();

        if let Some(source_circuit_id) = source_circuit_id
            && let Some(destination_circuit_id) = destination_circuit_id
        {
            // Both are already in the same circuit, nothing to do.
            if source_circuit_id == destination_circuit_id {
                return;
            }

            // Otherwise, we need to merge both circuit by moving all from destination circuit to source circuit.
            let destination_circuit_junction_boxes = circuit_junction_boxes
                .remove(&destination_circuit_id)
                .expect("Invalid state");
            for junction_box in &destination_circuit_junction_boxes {
                junction_box_circuit.insert(junction_box, source_circuit_id);
            }

            circuit_junction_boxes
                .get_mut(&source_circuit_id)
                .expect("Invalid state")
                .extend(destination_circuit_junction_boxes);
            return;
        }

        // Only source is currently part of a circuit.
        if let Some(source_circuit_id) = source_circuit_id {
            junction_box_circuit.insert(destination_junction_box, source_circuit_id);
            circuit_junction_boxes
                .get_mut(&source_circuit_id)
                .expect("Invalid state")
                .insert(destination_junction_box);
            return;
        }

        // Only destination is currently part of a circuit.
        if let Some(destination_circuit_id) = destination_circuit_id {
            junction_box_circuit.insert(source_junction_box, destination_circuit_id);
            circuit_junction_boxes
                .get_mut(&destination_circuit_id)
                .expect("Invalid state")
                .insert(source_junction_box);
            return;
        }

        // Otherwise, none are currently part of a circuit, so they will form their own circuit.

        junction_box_circuit.insert(source_junction_box, next_circuit_id);
        junction_box_circuit.insert(destination_junction_box, next_circuit_id);
        circuit_junction_boxes.insert(
            next_circuit_id,
            HashSet::from([source_junction_box, destination_junction_box]),
        );
    }

    fn sorted_junction_box_combinations(&self) -> Vec<(&Position, &Position)> {
        let junction_box_count = self.junction_boxes.len();
        let pair_count = (junction_box_count * (junction_box_count - 1)) / 2;
        let mut junction_box_combinations =
            HashSet::<(&Position, &Position)>::with_capacity(pair_count);

        for (i, source_junction_box) in self.junction_boxes.iter().enumerate() {
            for destination_junction_box in &self.junction_boxes[i + 1..] {
                if junction_box_combinations
                    .contains(&(source_junction_box, destination_junction_box))
                    || junction_box_combinations
                        .contains(&(destination_junction_box, source_junction_box))
                {
                    continue;
                }

                junction_box_combinations.insert((source_junction_box, destination_junction_box));
            }
        }

        let mut junction_box_combinations: Vec<_> = junction_box_combinations.into_iter().collect();

        junction_box_combinations.sort_by_key(|(first_position, second_position)| {
            first_position.euclidean_distance(second_position)
        });

        junction_box_combinations
    }
}

impl From<&str> for ElectricalGrid {
    fn from(value: &str) -> Self {
        let junction_boxes = value.lines().map(Position::from).collect();

        Self { junction_boxes }
    }
}
