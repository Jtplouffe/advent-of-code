use crate::{region::Region, shape::Shape};

mod region;
mod shape;

fn main() {
    let input = include_str!("./input").trim();

    let (shapes, regions) = input.rsplit_once("\n\n").expect("Invalid input");

    let shapes: Vec<_> = shapes.split("\n\n").map(Shape::from).collect();
    let regions: Vec<_> = regions.lines().map(Region::from).collect();

    let spacious_enough_region_count = regions
        .iter()
        .filter(|region| {
            // Shapes don't really fit togheter, this is good enough
            let max_required_space: usize = region
                .shapes_quantity
                .iter()
                .enumerate()
                .map(|(index, shape_quantity)| {
                    let max_size = shapes[index].width() * shapes[index].height();
                    shape_quantity * max_size
                })
                .sum();

            max_required_space <= region.width * region.height
        })
        .count();

    println!("Part 1: {spacious_enough_region_count}");
}
