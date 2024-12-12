use stone_line::StoneLine;

mod stone_line;

fn main() {
    let input = include_str!("./input").trim_end();

    let mut stone_line = StoneLine::from(input);

    println!("Part 1: {}", stone_line.stone_count_after_blinks(25));

    println!("Part 2: {}", stone_line.stone_count_after_blinks(75));
}
