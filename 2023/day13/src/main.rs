use pattern::Pattern;

mod pattern;
mod tile;

fn main() {
    let input = include_str!("./input");

    let patterns: Vec<_> = input.split("\n\n").map(Pattern::from).collect();

    let smudgeless_summary: usize = patterns.iter().map(|pattern| pattern.summary(0)).sum();
    println!("Part 1: {smudgeless_summary}");

    let summary_with_smudge: usize = patterns.iter().map(|pattern| pattern.summary(1)).sum();
    println!("Part 2: {summary_with_smudge}");
}
