use crate::category_converter_range::CategoryConverterRange;

pub struct CategoryConverter<'a> {
    pub source_category: &'a str,
    pub target_category: &'a str,

    ranges: Vec<CategoryConverterRange>,
}

impl<'a> CategoryConverter<'a> {
    pub fn convert(&self, source_number: usize) -> Option<usize> {
        for range in &self.ranges {
            if range.can_convert(source_number) {
                return Some(range.convert(source_number));
            }
        }

        None
    }
}

impl<'a> From<&'a str> for CategoryConverter<'a> {
    fn from(value: &'a str) -> Self {
        let mut lines = value.lines();
        let map = lines
            .next()
            .expect("expected map header")
            .split_whitespace()
            .next()
            .expect("expected white space");
        let (source_category, target_category) = map
            .split_once("-to-")
            .expect("expected source and target categories");

        let ranges = lines.map(CategoryConverterRange::from).collect();

        Self {
            source_category,
            target_category,
            ranges,
        }
    }
}
