fn main() {
    let input = include_str!("input");

    let rounds = input.lines().map(Round::from).collect::<Vec<_>>();

    let part1_total_score = rounds.iter().map(Round::part1_score).sum::<usize>();
    println!("Part 1: {part1_total_score}");

    let part2_total_score = rounds.iter().map(Round::part2_score).sum::<usize>();
    println!("Part 2: {part2_total_score}");
}

struct Round {
    action1: Action,
    part1_action: Action,
    part2_action: Action,
}

impl Round {
    fn part1_score(&self) -> usize {
        let action_score = if self.action1 == self.part1_action {
            3
        } else if self.part1_action.wins_over(&self.action1) {
            6
        } else {
            0
        };

        action_score + self.part1_action.score()
    }

    fn part2_score(&self) -> usize {
        let action_score = if self.action1 == self.part2_action {
            3
        } else if self.part2_action.wins_over(&self.action1) {
            6
        } else {
            0
        };

        action_score + self.part2_action.score()
    }
}

impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let (first, second) = s.split_once(" ").unwrap();
        let action1 = first.into();
        let part1_action = second.into();
        let part2_action = Action::from_opponent_action_and_outcome(&action1, &second.into());

        Self {
            action1,
            part1_action,
            part2_action,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn from_opponent_action_and_outcome(opponent_action: &Self, outcome: &Outcome) -> Self {
        match (outcome, opponent_action) {
            (&Outcome::Lose, &Self::Rock) => Self::Scissors,
            (&Outcome::Lose, &Self::Paper) => Self::Rock,
            (&Outcome::Lose, &Self::Scissors) => Self::Paper,

            (&Outcome::Draw, _) => opponent_action.to_owned(),

            (&Outcome::Win, &Self::Rock) => Self::Paper,
            (&Outcome::Win, &Self::Paper) => Self::Scissors,
            (&Outcome::Win, &Self::Scissors) => Self::Rock,
        }
    }

    fn wins_over(&self, other: &Self) -> bool {
        if self == &Self::Rock {
            other == &Self::Scissors
        } else if self == &Self::Paper {
            other == &Self::Rock
        } else {
            other == &Self::Paper
        }
    }

    fn score(&self) -> usize {
        match self {
            &Self::Rock => 1,
            &Self::Paper => 2,
            &Self::Scissors => 3,
        }
    }
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!()
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!()
        }
    }
}
