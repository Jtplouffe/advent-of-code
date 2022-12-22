use movement::Movement;

use crate::board::Board;

mod board;
mod cell;
mod movement;

fn main() {
    let input = include_str!("./input");

    let (board, movements) = input.split_once("\n\n").unwrap();
    let movements = Movement::movements_from_str(movements);

    let mut board = Board::from(board);
    println!("Part 1: {}", board.final_password(&movements));
}
