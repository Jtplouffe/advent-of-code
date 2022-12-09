use crate::forest::Forest;

mod forest;

fn main() {
    let input = include_str!("./input");

    let forest = Forest::from(input);

    let visible_trees_count = forest.visible_trees_count();
    println!("Part 1: {visible_trees_count}");

    let highest_scenic_score = forest.highest_scenic_score();
    println!("Part 2: {highest_scenic_score }");
}
