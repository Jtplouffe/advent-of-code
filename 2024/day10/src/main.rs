mod map;
mod position;

use map::Map;

fn main() {
    let input = include_str!("./input").trim_end();

    let map = Map::from(input);

    let trailhead_score_sum = map.trailhead_score_sum();
    println!("Part 1: {trailhead_score_sum}");

    let trailhead_rating_sum = map.trailhead_rating_sum();
    println!("Part 2: {trailhead_rating_sum}");
}
