use std::collections::VecDeque;

use crate::{indicator_lights::IndicatorLights, joltages::Joltages};

pub struct Machine {
    target_indicator_lights: IndicatorLights,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Joltages,
}

impl Machine {
    pub fn target_indicator_lights_fewest_button_presses(&self) -> usize {
        let mut fewest_press_count = usize::MAX;

        let mut queue = VecDeque::<(IndicatorLights, usize)>::new();
        queue.push_back((IndicatorLights::default(), 0));

        while let Some((current_indicator_lights, current_press_count)) = queue.pop_front() {
            // No point in continuing if always >= fewest press count
            if current_press_count >= fewest_press_count {
                continue;
            }

            // If matches target indicator lights, it is currently the fewest button presses
            if current_indicator_lights == self.target_indicator_lights {
                fewest_press_count = current_press_count;
                continue;
            }

            for button in &self.buttons {
                queue.push_back((
                    current_indicator_lights.copy_toggled(button),
                    current_press_count + 1,
                ));
            }
        }

        fewest_press_count
    }

    pub fn joltage_requirements_fewest_button_presses(&self) -> usize {
        let mut fewest_press_count = usize::MAX;

        let mut queue = VecDeque::<(Joltages, usize)>::new();
        queue.push_back((Joltages::default(), 0));

        while let Some((current_joltages, current_press_count)) = queue.pop_front() {
            // No point in continuing if always >= fewest press count
            if current_press_count >= fewest_press_count {
                continue;
            }

            // If matches target indicator lights, it is currently the fewest button presses
            if current_joltages == self.joltage_requirements {
                fewest_press_count = current_press_count;
                continue;
            }

            for button in &self.buttons {
                let new_joltages = current_joltages.copy_increased(button);
                if new_joltages.is_over_requirements(&self.joltage_requirements) {
                    continue;
                }

                queue.push_front((new_joltages, current_press_count + 1));
            }
        }

        fewest_press_count
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut target_indicator_lights = None;
        let mut buttons = Vec::new();
        let mut joltage_requirements = None;

        let extract_values = |part: &str| {
            part[1..part.len() - 1]
                .split(',')
                .map(|value| value.parse().unwrap())
                .collect::<Vec<usize>>()
        };

        for part in value.split(' ') {
            let first_char = part.chars().nth(0).unwrap();
            let last_char = part.chars().nth_back(0).unwrap();

            match (first_char, last_char) {
                ('[', ']') => target_indicator_lights = Some(IndicatorLights::from(part)),
                ('(', ')') => buttons.push(extract_values(part)),
                ('{', '}') => joltage_requirements = Some(Joltages::new(extract_values(part))),
                _ => unreachable!(),
            }
        }

        Self {
            target_indicator_lights: target_indicator_lights
                .expect("Failed to extract target indicator lights"),
            buttons,
            joltage_requirements: joltage_requirements
                .expect("Failed to extract joltage requirements"),
        }
    }
}
