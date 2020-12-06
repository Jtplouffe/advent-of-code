use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let answers: Vec<&str> = input.split("\n\n").collect();

    let part_1: usize = answers.iter().map(|answer| answer.replace("\n", ""))
        .map(|answer| answer.into_bytes().iter().collect::<HashSet<&u8>>().iter().map(|&a| *a as char).collect::<String>()
            .len()).sum();
    println!("Part 1: {}", part_1);

    let part_2: usize = answers.iter().map(|answer| {
        let lines: Vec<&str> = answer.split("\n").collect();
        let mut everyone = Vec::<u8>::new();
        for byte in lines.first().unwrap().as_bytes() {
            if answer.bytes().filter(|b| b == byte).count() >= lines.len() && !everyone.contains(byte) {
                everyone.push(*byte);
            }
        }
        everyone.len()
    }).sum();
    println!("Part 2: {}", part_2);
}
