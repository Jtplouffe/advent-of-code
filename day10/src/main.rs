use std::io::Write;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let mut adapters: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.push(*adapters.iter().max().unwrap() + 3);
    adapters.sort_unstable();
    let (mut one_jolt, mut three_jolt): (u16, u16) = (0, 0);
    for i in 1..adapters.len() {
        match adapters[i] - adapters[i - 1] {
            1 => one_jolt += 1,
            2 => nop,
            3 => three_jolt += 1,
            _ => unreachable!(),
        }
    }

    println!("Part 1: {}", one_jolt * three_jolt);

    adapters.remove(0);
    adapters.pop();
    // https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/10.rs
    let mut dp = HashMap::<i16, u64>::new();
    dp.insert(0, 1);
    for i in &adapters {
        let ans =
            dp.get(&(i - 1)).unwrap_or(&0) +
                dp.get(&(i - 2)).unwrap_or(&0) +
                dp.get(&(i - 3)).unwrap_or(&0);
        dp.insert(*i, ans);
    }

    println!("Part 2: {}", dp[&(*adapters.last().unwrap() as i16)]);
}