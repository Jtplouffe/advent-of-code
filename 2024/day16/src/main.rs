use maze::Maze;

mod maze;
mod orientation;
mod position;

fn main() {
    let input = include_str!("./input");

    let mut maze = Maze::from(input);

    let (lowest_score, tile_count) = maze.find_lowest_score();
    println!("Part 1: {lowest_score}");
    println!("Part 2: {tile_count}");
}
