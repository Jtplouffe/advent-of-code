use crate::bingo_board::BingoBoard;
use std::str::FromStr;

pub struct Bingo {
    numbers: Vec<u32>,
    current_number_index: usize,
    boards: Vec<BingoBoard>,
}

impl Bingo {
    pub fn first_board_to_win(&mut self) -> &BingoBoard {
        loop {
            for i in 0..self.boards.len() {
                self.boards[i].picked_number(self.numbers[self.current_number_index]);

                if self.boards[i].has_won {
                    return &self.boards[i];
                }
            }

            self.current_number_index += 1;
        }
    }

    pub fn last_board_to_win(&mut self) -> &BingoBoard {
        let mut won_count = 0;

        loop {
            for i in 0..self.boards.len() {
                if self.boards[i].has_won {
                    continue;
                }

                self.boards[i].picked_number(self.numbers[self.current_number_index]);

                if self.boards[i].has_won {
                    if won_count == self.boards.len() - 1 {
                        return &self.boards[i];
                    }

                    won_count += 1;
                }
            }

            self.current_number_index += 1;
        }
    }

    pub fn get_current_number(&self) -> u32 {
        self.numbers[self.current_number_index]
    }

    pub fn reset(&mut self) {
        self.current_number_index = 0;
        self.boards.iter_mut().for_each(BingoBoard::reset);
    }
}

impl From<&str> for Bingo {
    fn from(s: &str) -> Self {
        let (numbers, boards) = s.split_once("\n").unwrap();
        let numbers: Vec<_> = numbers
            .split(",")
            .map(|n| u32::from_str(n).unwrap())
            .collect();
        let boards = boards.split("\n\n").map(BingoBoard::from).collect();

        Self {
            numbers,
            current_number_index: 0,
            boards,
        }
    }
}
