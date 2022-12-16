use std::collections::HashSet;

use crate::sensor::Sensor;

pub struct Grid<'a> {
    sensors: &'a [Sensor],
}

impl<'a> Grid<'a> {
    pub fn new(sensors: &'a [Sensor]) -> Self {
        Self { sensors }
    }

    pub fn sensor_coverage_at_y(&self, y: isize) -> usize {
        let sensors = self
            .sensors
            .iter()
            .filter(|sensor| sensor.can_coverage_reach_y(y))
            .collect::<Vec<_>>();

        let coverage_on_axis = sensors
            .iter()
            .flat_map(|sensor| sensor.coverage_on_y_axis(y))
            .collect::<HashSet<_>>();

        let beacons_on_axis_count = sensors
            .iter()
            .filter_map(|sensor| {
                if sensor.beacon_position.y == y {
                    Some(sensor.beacon_position.x)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>()
            .len();

        coverage_on_axis.len() - beacons_on_axis_count
    }
}
