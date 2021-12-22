use std::collections::HashMap;
use std::str::FromStr;

use crate::player::Player;

pub struct DiracDice {
    pub player1: Player,
    pub player2: Player,
}

impl DiracDice {
    pub fn play_part1(&mut self) -> usize {
        let mut die = 0;
        let mut die_rolls = 0;

        loop {
            let mut value = 0;
            for _ in 0..3 {
                die_rolls += 1;
                die = (die % 100) + 1;
                value += die;
            }

            self.player1.play(value);
            if self.player1.score >= 1000 {
                return self.player2.score * die_rolls;
            }

            let mut value = 0;
            for _ in 0..3 {
                die_rolls += 1;
                die = (die % 100) + 1;
                value += die;
            }
            self.player2.play(value);
            if self.player2.score >= 1000 {
                return self.player1.score * die_rolls;
            }
        }
    }

    pub fn play_part2(&mut self) -> usize {
        let (p1_universes, p2_universes) = self.play_part2_recur(
            &mut HashMap::new(),
            0,
            0,
            self.player1.initial_position,
            self.player2.initial_position,
        );
        p1_universes.max(p2_universes)
    }

    // https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/21.rs
    fn play_part2_recur(
        &mut self,
        map: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
        p1_score: usize,
        p2_score: usize,
        p1_pos: usize,
        p2_pos: usize,
    ) -> (usize, usize) {
        if p2_score >= 21 {
            return (0, 1);
        }

        if let Some(&score) = map.get(&(p1_score, p2_score, p1_pos, p2_pos)) {
            return score;
        }

        let mut score = (0, 0);
        for (die, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let p1_pos = 1 + (p1_pos + die - 1) % 10;
            let (p1_score, p2_score) =
                self.play_part2_recur(map, p2_score, p1_score + p1_pos, p2_pos, p1_pos);
            score = (score.0 + p2_score * times, score.1 + p1_score * times);
        }

        map.insert((p1_score, p2_score, p1_pos, p2_pos), score);
        score
    }
}

impl From<&str> for DiracDice {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();

        let pos1 = usize::from_str(lines.next().unwrap().split_once(": ").unwrap().1).unwrap();
        let player1 = Player::new(pos1);

        let pos2 = usize::from_str(lines.next().unwrap().split_once(": ").unwrap().1).unwrap();
        let player2 = Player::new(pos2);

        Self { player1, player2 }
    }
}
