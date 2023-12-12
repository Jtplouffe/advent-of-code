use universe::Universe;

mod universe;

fn main() {
    let input = include_str!("./input");

    let universe = Universe::from(input);

    let shortest_pairs_distances_sum = universe.shortest_pairs_distances_sum(2);
    println!("Part 1: {shortest_pairs_distances_sum}");

    let shortest_pairs_distances_sum = universe.shortest_pairs_distances_sum(1_000_000);
    println!("Part 2: {shortest_pairs_distances_sum}");
}
