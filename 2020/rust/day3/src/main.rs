use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("assets/input").expect("Could not find file");
    let reader = BufReader::new(file);
    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.expect("Could not parse line").into_bytes()).collect();

    println!("Part 1: {}", count_trees(&lines, 3, 1));
    println!("Part 2: {}", {
       let mut trees_prod = 1;

        trees_prod *= count_trees(&lines, 1, 1);
        trees_prod *= count_trees(&lines, 3, 1);
        trees_prod *= count_trees(&lines, 5, 1);
        trees_prod *= count_trees(&lines, 7, 1);
        trees_prod *= count_trees(&lines, 1, 2);

        trees_prod
    })
}

fn count_trees(lines: &Vec<Vec<u8>>, step_x: i32, step_y: i32) -> i64 {
    let mut trees: i64 = 0;
    let mut x = 0;
    for y in (0..lines.len()).step_by(step_y as usize) {
        let line = lines.get(y).expect("Line out of band");
        let char = line.get(x).expect("Char out of band");

        x += step_x as usize;
        x %= line.len();
        if *char as char == '#' {
            trees += 1;
        }
    }
    trees
}