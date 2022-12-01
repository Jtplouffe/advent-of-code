fn main() {
    let input = include_str!("./input");

    let mut calories_by_elf = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|calory| calory.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    calories_by_elf.sort_by(|a, b| b.cmp(a));

    let max_coleries = calories_by_elf.iter().max().unwrap();
    println!("Part 1: {max_coleries}");

    let top_three_elf_calories_sum = calories_by_elf.iter().take(3).sum::<u32>();
    println!("Part 2: {top_three_elf_calories_sum}");
}
