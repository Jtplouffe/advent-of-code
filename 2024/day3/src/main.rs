use regex::Regex;

fn main() {
    let input = include_str!("./input");

    let sum_without_conditions = execute_program_without_conditions(input);
    println!("Part 1: {sum_without_conditions}");

    let sum_with_conditions = execute_program_with_conditions(input);
    println!("Part 2: {sum_with_conditions}");
}

fn execute_program_without_conditions(program: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for capture in regex.captures_iter(program) {
        let left: i32 = capture[1].parse().unwrap();
        let right: i32 = capture[2].parse().unwrap();

        sum += left * right;
    }

    sum
}

fn execute_program_with_conditions(program: &str) -> i32 {
    let regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for capture in regex.captures_iter(program) {
        match &capture[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                let left: i32 = capture[2].parse().unwrap();
                let right: i32 = capture[3].parse().unwrap();

                sum += left * right;
            }
            _ => {}
        };
    }

    sum
}
