#[derive(Clone, Copy, PartialEq)]
pub enum InvalidIdMode {
    SingleRepetition,
    MultipleRepetition,
}

pub struct Range {
    first_id: usize,
    last_id: usize,
}

impl Range {
    pub fn invalid_ids_sum(&self, mode: InvalidIdMode) -> usize {
        self.iter().filter(|&id| !Self::is_id_valid(id, mode)).sum()
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
        self.first_id..=self.last_id
    }

    fn is_id_valid(id: usize, mode: InvalidIdMode) -> bool {
        match mode {
            InvalidIdMode::SingleRepetition => Self::is_id_valid_single_repetition(id),
            InvalidIdMode::MultipleRepetition => Self::is_id_valid_multiple_repetition(id),
        }
    }

    fn is_id_valid_single_repetition(id: usize) -> bool {
        let id_str = id.to_string();
        if !id_str.len().is_multiple_of(2) {
            return true;
        }

        let mid = id_str.len() / 2;
        id_str[0..mid] != id_str[mid..]
    }

    fn is_id_valid_multiple_repetition(id: usize) -> bool {
        let id_str = id.to_string();
        // A sequence can be at most half the id, otherwise not enough chars for other sequence
        'sequencer: for base_sequence_end in 0..(id_str.len() / 2) {
            let base_sequence = &id_str[..=base_sequence_end];

            // No point in checking if id len is not a multiple if base sequence length
            if !id_str.len().is_multiple_of(base_sequence.len()) {
                continue;
            }

            for next_sequence_start in
                ((base_sequence_end + 1)..id_str.len()).step_by(base_sequence.len())
            {
                let sequence =
                    &id_str[next_sequence_start..=next_sequence_start + base_sequence_end];

                if sequence != base_sequence {
                    continue 'sequencer;
                }
            }

            // If this is reached, it means that the sequence matching logic was never broken,
            // therefore the whole id is made up of sequences
            return false;
        }

        true
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (first_id, last_id) = value.split_once('-').expect("Invalid range");
        let first_id = first_id.parse().expect("Invalid first id");
        let last_id = last_id.parse().expect("Invalid last id");

        Self { first_id, last_id }
    }
}
