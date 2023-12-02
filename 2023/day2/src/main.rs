use game::{CubeColor, Game};

mod game;

fn main() {
    let input = include_str!("./input");

    let games: Vec<_> = input.lines().map(Game::from).collect();

    let part_1_cubes: &[(CubeColor, usize)] = &[
        (CubeColor::Red, 12),
        (CubeColor::Green, 13),
        (CubeColor::Blue, 14),
    ];
    let part_1: usize = games
        .iter()
        .filter(|game| game.are_cubes_possible(part_1_cubes))
        .map(|game| game.id)
        .sum();
    println!("Part 1: {part_1}");

    let game_power_sum: usize = games
        .iter()
        .map(Game::minimun_colored_cubes_qauntity_power)
        .sum();
    println!("Part 2: {game_power_sum}");
}
