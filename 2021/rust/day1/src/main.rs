fn main() {
    let input = include_str!("input");
    let depths: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut part_one_increases = 0;
    for i in 1..depths.len() {
        if depths[i - 1] < depths[i] {
            part_one_increases += 1;
        }
    }

    println!("Part 1: {}", part_one_increases);

    let mut part_two_increases = 0;
    for i in 3..depths.len() {
        if depths[i - 3] + depths[i - 2] + depths[i - 1] < depths[i - 2] + depths[i - 1] + depths[i] {
            part_two_increases += 1;
        }
    }

    println!("Part 2: {}", part_two_increases);
}
