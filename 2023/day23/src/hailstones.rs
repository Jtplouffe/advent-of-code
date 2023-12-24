use crate::hailstone::HailStone;

pub struct HailStones {
    hailstones: Vec<HailStone>,
}

impl HailStones {
    pub fn intersections_within_test_area_2d(
        &self,
        test_area_start: (f64, f64),
        test_area_end: (f64, f64),
    ) -> usize {
        let mut intersection_count = 0;

        for hailstone_index in 0..self.hailstones.len() {
            let hailstone = &self.hailstones[hailstone_index];

            for other_hailstone_index in hailstone_index + 1..self.hailstones.len() {
                let other_hailstone = &self.hailstones[other_hailstone_index];

                match hailstone.intersection_2d(other_hailstone) {
                    Some((x, y))
                        if x >= test_area_start.0
                            && x <= test_area_end.0
                            && y >= test_area_start.1
                            && y <= test_area_end.1
                            && hailstone.is_point_in_future((x, y))
                            && other_hailstone.is_point_in_future((x, y)) =>
                    {
                        intersection_count += 1;
                    }
                    _ => {}
                };
            }
        }

        intersection_count
    }
}

impl From<&str> for HailStones {
    fn from(value: &str) -> Self {
        let hailstones = value.lines().map(HailStone::from).collect();

        Self { hailstones }
    }
}
