use std::ops::Sub;

fn main() {
    let input = include_str!("./input");

    let location_ids: Vec<(i32, i32)> = input
        .trim()
        .lines()
        .map(|line| {
            let (first, second) = line.split_once("   ").unwrap();
            return (first.parse().unwrap(), second.parse().unwrap());
        })
        .collect();

    let mut left_location_ids: Vec<_> = location_ids
        .iter()
        .map(|(location_id, _)| *location_id)
        .collect();

    let mut right_location_ids: Vec<_> = location_ids
        .iter()
        .map(|(_, location_id)| *location_id)
        .collect();

    left_location_ids.sort();
    right_location_ids.sort();

    let total_distance = calculate_total_distance(&right_location_ids, &left_location_ids);
    println!("Part 1: {total_distance}");

    let total_similarity_score =
        calculate_total_similarity_score(&left_location_ids, &right_location_ids);
    println!("Part 2: {total_similarity_score}");
}

fn calculate_total_distance(left_location_ids: &[i32], right_location_ids: &[i32]) -> i32 {
    let distances: Vec<_> = left_location_ids
        .iter()
        .zip(right_location_ids)
        .map(|(left, right)| left.sub(right).abs())
        .collect();

    distances.iter().sum()
}

fn calculate_total_similarity_score(left_location_ids: &[i32], right_location_ids: &[i32]) -> i32 {
    let similarity_scores = left_location_ids.iter().map(|&location_id| {
        let count = right_location_ids
            .iter()
            .filter(|&&right| right == location_id)
            .count();

        location_id * count as i32
    });

    similarity_scores.sum()
}
