pub struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    pub fn new(time: usize, record_distance: usize) -> Self {
        Self {
            time,
            record_distance,
        }
    }

    pub fn possible_ways_to_beat_record(&self) -> usize {
        let breaking_record_from = (0..self.time)
            .find(|&button_hold_time| self.is_button_hold_time_record_breaking(button_hold_time));
        let breaking_record_until = (0..self.time)
            .rev()
            .find(|&button_hold_time| self.is_button_hold_time_record_breaking(button_hold_time));

        match (breaking_record_from, breaking_record_until) {
            (Some(from), Some(to)) => to - from + 1,
            _ => 0,
        }
    }

    fn is_button_hold_time_record_breaking(&self, button_hold_time: usize) -> bool {
        let time_remaining = self.time - button_hold_time;
        let distance_traveled = time_remaining * button_hold_time;

        distance_traveled > self.record_distance
    }
}
