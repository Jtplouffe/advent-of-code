use crate::tachyon_manifold::TachyonManifold;

mod position;
mod tachyon_manifold;

fn main() {
    let input = include_str!("./input").trim();
    let tachion_manifold = TachyonManifold::from(input);

    let split_count = tachion_manifold.split_count();
    println!("Part 1: {split_count}");

    let split_count_quantum = tachion_manifold.split_count_quantum();
    println!("Part 2: {split_count_quantum}");
}
