use crate::cave::Cave;

mod cave;
mod dijkstra_state;

fn main() {
    let input = include_str!("input");
    let mut cave = Cave::from(input);

    println!("Part one: {}", cave.lowest_risk_level());

    cave.expand_risk_levels(5);
    println!("Part two: {}", cave.lowest_risk_level());
}
