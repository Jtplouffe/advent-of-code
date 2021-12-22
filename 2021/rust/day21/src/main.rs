use crate::dirac_dice::DiracDice;

mod dirac_dice;
mod player;

fn main() {
    let input = include_str!("input");
    let mut dirac_dice = DiracDice::from(input);

    println!("Part one: {}", dirac_dice.play_part1());
    println!("Part two: {}", dirac_dice.play_part2());
}
