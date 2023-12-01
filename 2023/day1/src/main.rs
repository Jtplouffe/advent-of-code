use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref DIGIT_NAMES: HashMap<&'static str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    static ref MAX_DIGIT_NAME_LENGTH: usize =
        DIGIT_NAMES.keys().map(|name| name.len()).max().unwrap_or(0);
}

fn main() {
    let input = include_str!("./input");

    let calibration_numbers: Vec<_> = input
        .lines()
        .filter_map(extract_calibration_number)
        .collect();

    let calibration_numbers_sum: usize = calibration_numbers.iter().sum();
    println!("Part 1: {calibration_numbers_sum}");

    let normalized_calibration_numbers: Vec<_> = input
        .lines()
        .map(normalize_line_spelled_digits)
        .filter_map(|line| extract_calibration_number(&line))
        .collect();
    let normalized_calibration_numbers_sum: usize = normalized_calibration_numbers.iter().sum();
    println!("Part 2: {normalized_calibration_numbers_sum}");
}

fn extract_calibration_number(line: &str) -> Option<usize> {
    let first_digit = line.chars().find(char::is_ascii_digit)?;
    let last_digit = line.chars().rev().find(char::is_ascii_digit)?;

    let calibration_number = format!("{first_digit}{last_digit}").parse().ok()?;

    Some(calibration_number)
}

fn normalize_line_spelled_digits(line: &str) -> String {
    let mut normalized_line = line.to_string();

    let mut i = 0;
    while i < normalized_line.len() {
        let mut j = i;
        while j < normalized_line.len() && j - i < *MAX_DIGIT_NAME_LENGTH {
            if let Some(&digit) = DIGIT_NAMES.get(&normalized_line[i..=j]) {
                normalized_line.insert(i + ((j - i) / 2), digit);
                break;
            }
            j += 1
        }

        i += 1
    }

    normalized_line
}
