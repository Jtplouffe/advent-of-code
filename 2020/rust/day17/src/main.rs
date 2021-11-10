use crate::pocket_dimension_3d::PocketDimension3D;
use crate::pocket_dimension_4d::PocketDimension4D;

mod pocket_dimension_3d;
mod pocket_dimension_4d;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let mut pocket_dimension_3d = PocketDimension3D::from(input.as_str());
    let mut pocket_dimension_4d = PocketDimension4D::from(input.as_str());

    (0..6).for_each(|_| {
        pocket_dimension_3d.run();
        pocket_dimension_4d.run();
    });

    println!("Part 1: {}", pocket_dimension_3d.active_count());
    println!("Part 2: {}", pocket_dimension_4d.active_count());
}
