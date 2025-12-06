use std::{mem::swap, ops::RangeInclusive};

pub struct IngredientDatabase {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl IngredientDatabase {
    pub fn fresh_ingredient_count(&self) -> usize {
        let mut count = 0;

        for ingredient in &self.ingredients {
            for fresh_range in &self.fresh_ranges {
                if fresh_range.contains(ingredient) {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    pub fn total_possible_fresh_ingredient_count(&self) -> usize {
        let mut distinct_ranges =
            Vec::<RangeInclusive<usize>>::with_capacity(self.fresh_ranges.len());

        for fresh_range in &self.fresh_ranges {
            let mut new_distinct_ranges = vec![fresh_range.to_owned()];
            let mut pending_new_distanct_ranges = Vec::new();

            for distinct_range in &distinct_ranges {
                for new_distanct_range in &new_distinct_ranges {
                    pending_new_distanct_ranges
                        .extend(deintersect_range(distinct_range, new_distanct_range));
                }

                swap(&mut new_distinct_ranges, &mut pending_new_distanct_ranges);
                pending_new_distanct_ranges.clear();
            }

            distinct_ranges.extend(new_distinct_ranges);
        }

        distinct_ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
    }
}

impl From<&str> for IngredientDatabase {
    fn from(value: &str) -> Self {
        let (fresh_ranges, ingredients) = value.split_once("\n\n").expect("Invalid input");

        let fresh_ranges = fresh_ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').expect("Invalid range");
                let start = start.parse().expect("Invaild start");
                let end = end.parse().expect("Invalid end");

                debug_assert!(start <= end);

                start..=end
            })
            .collect();

        let ingredients = ingredients
            .lines()
            .map(|ingredient| ingredient.parse().expect("Invalid ingredient"))
            .collect();

        Self {
            fresh_ranges,
            ingredients,
        }
    }
}

fn deintersect_range(
    reference: &RangeInclusive<usize>,
    other: &RangeInclusive<usize>,
) -> Vec<RangeInclusive<usize>> {
    let start = *other.start();
    let end = *other.end();

    let mut distinct_ranges = Vec::new();
    if &start < reference.start() {
        let end = end.min(reference.start() - 1);
        distinct_ranges.push(start..=end);
    }

    if &end > reference.end() {
        let start = start.max(reference.end() + 1);
        distinct_ranges.push(start..=end);
    }

    distinct_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    mod deintersect_range {
        use super::*;

        #[test]
        fn no_overlap_smaller() {
            // Arrange
            let reference = 10..=20;
            let other = 0..=10;

            // Act
            let actual = deintersect_range(&reference, &other);

            // Assert
            assert_eq!(actual, vec![0..=9]);
        }

        #[test]
        fn no_overlap_greater() {
            // Arrange
            let reference = 10..=20;
            let other = 21..=23;

            // Act
            let actual = deintersect_range(&reference, &other);

            // Assert
            assert_eq!(actual, vec![21..=23]);
        }

        #[test]
        fn fully_within_ref() {
            // Arrange
            let reference = 10..=20;
            let other = 13..=19;

            // Act
            let actual = deintersect_range(&reference, &other);

            // Assert
            assert_eq!(actual, vec![]);
        }

        #[test]
        fn same_range() {
            // Arrange
            let reference = 10..=20;
            let other = 10..=20;

            // Act
            let actual = deintersect_range(&reference, &other);

            // Assert
            assert_eq!(actual, vec![]);
        }

        #[test]
        fn full_overlap() {
            // Arrange
            let reference = 10..=20;
            let other = 5..=25;

            // Act
            let actual = deintersect_range(&reference, &other);

            // Assert
            assert_eq!(actual, vec![5..=9, 21..=25]);
        }

        #[test]
        fn left_overlap() {
            // Arrange
            let reference = 10..=20;
            let other = 5..=15;

            // Act
            let actual = deintersect_range(&reference, &other);

            // Assert
            assert_eq!(actual, vec![5..=9]);
        }

        #[test]
        fn right_overlap() {
            // Arrange
            let reference = 10..=20;
            let other = 15..=25;

            // Act
            let actual = deintersect_range(&reference, &other);

            // Assert
            assert_eq!(actual, vec![21..=25]);
        }
    }
}
