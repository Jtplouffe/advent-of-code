fn main() {
    let input = include_str!("./input");

    let communication_system: CommunicationSystem = input.into();

    let start_of_packet = communication_system.characters_processed_before_packet(4);
    println!("Part 1: {start_of_packet}");

    let start_of_message = communication_system.characters_processed_before_packet(14);
    println!("Part 2: {start_of_message}");
}

struct CommunicationSystem<'a> {
    data_stream: &'a str,
}

impl<'a> CommunicationSystem<'a> {
    fn characters_processed_before_packet(&self, length: usize) -> usize {
        for data_stream_index in (length - 1)..self.data_stream.len() {
            let mut current_sequence: Vec<char> = Vec::with_capacity(length);
            let mut current_sequence_chars_unique = true;

            for sequence_char in self.data_stream[data_stream_index - (length - 1)..=data_stream_index].chars() {
                if current_sequence.contains(&sequence_char) {
                    current_sequence_chars_unique  = false;
                    break;
                }

                current_sequence.push(sequence_char);
            }

            if current_sequence_chars_unique {
                return data_stream_index + 1;
            }
        }

        unreachable!()
    }
}

impl<'a> From<&'a str> for CommunicationSystem<'a> {
    fn from(s: &'a str) -> Self {
        Self { data_stream: s }
    }
}