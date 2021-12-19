use crate::snailfish_number::SnailfishNumber;

mod snailfish_number;

fn main() {
    let input = include_str!("input");
    let snailfish_numbers: Vec<_> = input.lines().map(SnailfishNumber::from).collect();

    let mut total = snailfish_numbers.get(0).unwrap().clone();
    for s in snailfish_numbers[1..].iter() {
        total = total.clone() + s.clone();
    }
    println!("Part one: {}", total.get_magnitude());

    let mut magnitudes = Vec::new();
    for (a_index, a) in snailfish_numbers.iter().enumerate() {
        for b in snailfish_numbers[a_index..].iter() {
            magnitudes.push((a.clone() + b.clone()).get_magnitude());
            magnitudes.push((b.clone() + a.clone()).get_magnitude());
        }
    }
    println!("Part two: {}", magnitudes.iter().max().unwrap());
}
