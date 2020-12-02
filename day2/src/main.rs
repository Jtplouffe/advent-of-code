use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("assets/input").expect("Could not find file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();

    println!("Part 1: {}", lines.iter().fold(0, |total, line| {
        let (pw, letter, min, max) = process_line(line);

        let letter_count = pw.matches(letter).count() as i32;
        if letter_count >= min && letter_count <= max { total + 1 } else { total }
    }));

    println!("Part 2: {}", lines.iter().fold(0, |total, line| {
        let (pw, letter, first_pos, last_pos) = process_line(line);

        let letter = letter.as_bytes().get(0).unwrap();

        let first = pw.as_bytes().get(first_pos as usize - 1);
        let second = pw.as_bytes().get(last_pos as usize - 1);

        if (first.is_some() && first.unwrap() == letter) != (second.is_some() && second.unwrap() == letter) {
            return total + 1;
        }

        total
    }));
}

fn process_line(line: &str) -> (&str, &str, i32, i32) {
    let split_line: Vec<&str> = line.split(": ").collect();
    let policy = split_line.first().unwrap();
    let pw = split_line.last().unwrap();

    let split_policy: Vec<&str> = policy.split(" ").collect();
    let letter = split_policy.last().unwrap();
    let min_max: Vec<&str> = split_policy.first().unwrap().split("-").collect();
    let first_nb = min_max.first().unwrap().parse::<i32>().unwrap();
    let last_nb = min_max.last().unwrap().parse::<i32>().unwrap();

    (pw, letter, first_nb, last_nb)
}