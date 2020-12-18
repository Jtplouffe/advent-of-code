#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};
use std::ops::{RangeInclusive};

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let split_input: Vec<_> = input.split("\n\n").collect();
    let fields = parse_fields(split_input[0]);

    let my_ticket: Vec<usize> = split_input[1]
        .lines()
        .last().unwrap()
        .split(",").map(|v| v.parse().unwrap()).collect();
    let mut nearby_tickets: Vec<Vec<usize>> = split_input[2].lines().skip(1)
        .map(|line| line.split(",")
            .map(|v| v.parse().unwrap())
            .collect()
        ).collect();

    println!("Part 1: {}", ticket_scanning_error_rate(&fields, &nearby_tickets));

    filter_out_invalid_ticket(&fields, &mut nearby_tickets);

    println!("Part 2: {}", deduct_values(&fields, &my_ticket, &nearby_tickets));
}

fn parse_fields(raw_fields: &str) -> HashMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)> {
    let mut fields = HashMap::new();

    for line in raw_fields.lines() {
        let (field_name, values) = line.split_once(": ").unwrap();
        let ((first_min, first_max), (second_min, second_max)) = values.split_once(" or ")
            .and_then(|(a, b)| Some((a.split_once("-").unwrap(), b.split_once("-").unwrap()))).unwrap();

        fields.insert(field_name, (
            first_min.parse().unwrap()..=first_max.parse().unwrap(),
            second_min.parse().unwrap()..=second_max.parse().unwrap()
        ));
    }

    fields
}

fn ticket_scanning_error_rate(fields: &HashMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>, tickets: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;

    for v in tickets.iter().flatten() {
        if !fields.iter().any(|f| f.1.0.contains(v) || f.1.1.contains(v)) {
            sum += v;
        }
    }

    sum
}

fn filter_out_invalid_ticket(fields: &HashMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>, tickets: &mut Vec<Vec<usize>>) {
    let mut invalid_ticket_index = Vec::<usize>::new();
    for (i, v) in tickets.iter().flatten().enumerate() {
        if !fields.iter().any(|f| f.1.0.contains(v) || f.1.1.contains(v)) {
            invalid_ticket_index.push(i / tickets[0].len());
            continue;
        }
    }

    invalid_ticket_index.sort_by(|a, b| b.cmp(a));

    for i in invalid_ticket_index {
        tickets.remove(i);
    }
}

// Stolen from https://dev.to/benwtrent/comment/198on
fn deduct_values(
    fields: &HashMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>,
    my_ticket: &Vec<usize>,
    nearby_tickets: &Vec<Vec<usize>>,
) -> usize {
    let valid_tickets: Vec<&Vec<usize>> = nearby_tickets
        .iter()
        .filter(|&vec| {
            !vec.iter()
                .any(|v| fields.iter().all(|f| !f.1.0.contains(&v) && !f.1.1.contains(&v)))
        })
        .collect();

    let mut field_translation = HashMap::new();
    let mut translated_fields = HashSet::new();
    while field_translation.len() < fields.len() {
        for f in fields.iter() {
            if translated_fields.contains(&f) {
                continue;
            }
            let mut matched = false;
            let mut matched_more = false;
            let mut column = 0;
            for j in 0..fields.len() {
                if field_translation.contains_key(&j) {
                    continue;
                }
                if !valid_tickets.iter().any(|vec| !f.1.0.contains(&vec[j]) && !f.1.1.contains(&vec[j])) {
                    if matched {
                        matched_more = true;
                        break;
                    }
                    matched = true;
                    column = j;
                }
            }
            if matched && !matched_more {
                field_translation.insert(column, f);
                translated_fields.insert(f);
            }
        }
    }

    my_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| field_translation[i].0.contains("departure"))
        .map(|(_, v)| v)
        .product()
}