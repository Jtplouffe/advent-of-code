use crate::bingo::Bingo;

mod bingo;
mod bingo_board;

fn main() {
    let input = include_str!("input");
    let mut bingo = Bingo::from(input);

    let first_board_to_win = bingo.first_board_to_win();
    let first_board_to_win_score = first_board_to_win.unmarked_numbers_sum() * bingo.get_current_number();
    println!("Part one: {}", first_board_to_win_score);

    bingo.reset();

    let last_board_to_win = bingo.last_board_to_win();
    let last_board_to_win_score = last_board_to_win.unmarked_numbers_sum() * bingo.get_current_number();
    println!("Part two: {}", last_board_to_win_score);
}
