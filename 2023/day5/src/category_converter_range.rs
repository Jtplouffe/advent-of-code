pub struct CategoryConverterRange {
    source_range_start: usize,
    source_range_end: usize,

    transposition: isize,
}

impl CategoryConverterRange {
    pub fn can_convert(&self, source_number: usize) -> bool {
        source_number >= self.source_range_start && source_number <= self.source_range_end
    }

    pub fn convert(&self, source_number: usize) -> usize {
        (source_number as isize + self.transposition) as usize
    }
}

impl From<&str> for CategoryConverterRange {
    fn from(value: &str) -> Self {
        let mut mapped_values = value
            .split_whitespace()
            .map(|value| value.parse().expect("expected usize"));

        let destination_range_start = mapped_values.nth(0).expect("expected a value");
        let source_range_start = mapped_values.nth(0).expect("expected a value");
        let range = mapped_values.nth(0).expect("expected a value");

        let transposition = destination_range_start as isize - source_range_start as isize;

        Self {
            source_range_start,
            source_range_end: source_range_start + range - 1,
            transposition,
        }
    }
}
