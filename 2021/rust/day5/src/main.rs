use std::collections::HashMap;

use crate::line::Line;
use crate::point::Point;

mod line;
mod point;

fn main() {
    let input = include_str!("input");

    let lines: Vec<_> = input.lines().map(Line::from).collect();
    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<Line>) {
    let points: Vec<_> = lines
        .iter()
        .filter(|l| l.is_linear())
        .map(Line::get_points)
        .flat_map(|lp| lp)
        .collect();

    let mut points_map = HashMap::<&Point, u32>::new();
    for point in &points {
        match points_map.get(point) {
            Some(&count) => points_map.insert(point, count + 1),
            None => points_map.insert(point, 1),
        };
    }

    let point_overlap_count = points_map.values().into_iter().filter(|&v| *v > 1).count();

    println!("Part one: {:?}", point_overlap_count)
}

fn part_two(lines: &Vec<Line>) {
    let points: Vec<_> = lines.iter().map(Line::get_points).flat_map(|lp| lp).collect();

    let mut points_map = HashMap::<&Point, u32>::new();
    for point in &points {
        match points_map.get(point) {
            Some(&count) => points_map.insert(point, count + 1),
            None => points_map.insert(point, 1),
        };
    }

    let point_overlap_count = points_map.values().into_iter().filter(|&v| *v > 1).count();

    println!("Part two: {}", point_overlap_count);
}