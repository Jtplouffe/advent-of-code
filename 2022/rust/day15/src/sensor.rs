use crate::position::Position;

pub struct Sensor {
    pub sensor_position: Position,
    pub beacon_position: Position,
}

impl Sensor {
    pub fn can_coverage_reach_y(&self, y: isize) -> bool {
        let radius = self.radius();

        self.sensor_position.y - radius <= y && self.sensor_position.y + radius >= y
    }

    pub fn radius(&self) -> isize {
        self.sensor_position.x.max(self.beacon_position.x)
            - self.sensor_position.x.min(self.beacon_position.x)
            + self.sensor_position.y.max(self.beacon_position.y)
            - self.sensor_position.y.min(self.beacon_position.y)
    }

    pub fn coverage_on_y_axis(&self, y: isize) -> impl Iterator<Item = isize> {
        let radius = self.radius();
        let distance_to_sensor = (self.sensor_position.y - y).abs();

        self.sensor_position.x - (radius - distance_to_sensor)
            ..=self.sensor_position.x + (radius - distance_to_sensor)
    }
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        let s = s
            .replace("Sensor at ", "")
            .replace(" closest beacon is at ", "");
        let (sensor_position, beacon_position) = s.split_once(":").unwrap();

        Self {
            sensor_position: sensor_position.into(),
            beacon_position: beacon_position.into(),
        }
    }
}
