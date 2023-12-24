use hailstones::HailStones;

mod hailstone;
mod hailstones;

fn main() {
    let input = include_str!("./input");

    let hailstones = HailStones::from(input);

    let intersection_count = hailstones.intersections_within_test_area_2d(
        (200000000000000.0, 200000000000000.0),
        (400000000000000.0, 400000000000000.0),
    );
    println!("Part 1: {intersection_count}");
}
