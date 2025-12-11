#[derive(Default, Clone, Eq, PartialEq)]
pub struct IndicatorLights(u16);

impl IndicatorLights {
    pub fn copy_toggled(&self, toggle_indexes: &[usize]) -> Self {
        let mut value = self.0;

        for toggle_index in toggle_indexes {
            value ^= 1 << toggle_index;
        }

        Self(value)
    }
}

impl From<&str> for IndicatorLights {
    fn from(str: &str) -> Self {
        let mut value = 0;

        let mut current_index = 0;
        for char in str.chars() {
            match char {
                '[' if current_index == 0 => {}
                '.' => {}
                '#' => value |= 1 << (current_index - 1),
                ']' => return Self(value),
                _ => break,
            }

            current_index += 1;
        }

        panic!("Malformed indicator lights")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn from_invalid() {
        let _ = IndicatorLights::from(".##.");
    }

    #[test]
    fn from_empty() {
        let indicator_lights = IndicatorLights::from("[]");

        assert_eq!(indicator_lights.0, 0b0);
    }

    #[test]
    fn from_one_off() {
        let indicator_lights = IndicatorLights::from("[.]");

        assert_eq!(indicator_lights.0, 0b0);
    }

    #[test]
    fn from_off() {
        let indicator_lights = IndicatorLights::from("[.]");

        assert_eq!(indicator_lights.0, 0b0);
    }

    #[test]
    fn from_on() {
        let indicator_lights = IndicatorLights::from("[#]");

        assert_eq!(indicator_lights.0, 0b1);
    }

    #[test]
    fn from_off_on() {
        let indicator_lights = IndicatorLights::from("[.#]");

        assert_eq!(indicator_lights.0, 0b10);
    }

    #[test]
    fn from_on_on() {
        let indicator_lights = IndicatorLights::from("[##]");

        assert_eq!(indicator_lights.0, 0b11);
    }

    #[test]
    fn from_off_off_on() {
        let indicator_lights = IndicatorLights::from("[..#]");

        assert_eq!(indicator_lights.0, 4);
    }

    #[test]
    fn from_off_on_off() {
        let indicator_lights = IndicatorLights::from("[.##]");

        assert_eq!(indicator_lights.0, 0b110);
    }

    #[test]
    fn copy_toggled() {
        let indicator_lights = IndicatorLights(0b10110).copy_toggled(&[0, 2, 3, 5, 7]);

        assert_eq!(indicator_lights.0, 0b10111011);
    }
}
