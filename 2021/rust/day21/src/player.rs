pub struct Player {
    pub initial_position: usize,
    pub position: usize,
    pub score: usize,
}

impl Player {
    pub fn new(initial_position: usize) -> Self {
        Self {
            initial_position,
            position: initial_position,
            score: 0,
        }
    }

    pub fn play(&mut self, die_value: usize) {
        self.position = (self.position + die_value - 1) % 10 + 1;
        self.score += self.position as usize;
    }
}
