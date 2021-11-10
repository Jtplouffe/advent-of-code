use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("assets/input").expect("Could not find file");
    let reader = BufReader::new(file);
    let lines: Vec<i32> = reader.lines()
        .map(|l| {
            l.expect("Could not parse line").parse::<i32>().expect("Could not convert to i32")
        })
        .collect();

    for i in 0..lines.len() {
        for n in i + 1..lines.len() {
            let a = lines.get(i).unwrap();
            let b = lines.get(n).unwrap();

            if a + b == 2020 {
                println!("Part 1: {}", a * b);
            }
        }
    }

    for i in 0..lines.len() {
        for n in i + 1..lines.len() {
            for m in n + 1..lines.len() {
                let a = lines.get(i).unwrap();
                let b = lines.get(n).unwrap();
                let c = lines.get(m).unwrap();

                if a + b + c == 2020 {
                    println!("Part 2: {}", a * b * c);
                }
            }
        }
    }
}
