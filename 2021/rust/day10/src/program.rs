use crate::program_line::ProgramLine;

pub struct Program {
    pub lines: Vec<ProgramLine>,
}

impl Program {
    pub fn get_syntax_error_score(&self) -> u32 {
        self.lines
            .iter()
            .filter_map(|line| {
                if let Some(instruction) = line.first_corrupted_instruction() {
                    Some(instruction.syntax_error_score())
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn autocomplete_score(&self) -> u64 {
        let mut scores: Vec<_> = self
            .lines
            .iter()
            .filter_map(|line| line.autocomplete_score())
            .collect();
        scores.sort();

        scores[(scores.len() as f32 / 2f32).floor() as usize]
    }
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        let lines = s.lines().map(ProgramLine::from).collect();

        Self { lines }
    }
}
