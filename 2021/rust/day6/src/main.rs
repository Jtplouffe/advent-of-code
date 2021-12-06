use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("input");

    let fishes: Vec<_> = input.split(",").filter_map(|n| u8::from_str(n).ok()).collect();

    println!("Part one: {}", get_fish_count_after_days(&mut fishes.clone(), 80));
    println!("Part two: {}", get_fish_count_after_days(&mut fishes.clone(), 256));
}

fn get_fish_count_after_days(fishes: &mut Vec<u8>, days: usize) -> u64 {
    let mut fishes_map = HashMap::<u8, u64>::new();
    for f in fishes {
        match fishes_map.get(f) {
            Some(&count) => fishes_map.insert(*f, count + 1),
            None => fishes_map.insert(*f, 1),
        };
    }

    for _ in 0..days {
        let mut new_fishes_map = HashMap::<u8, u64>::new();
        for (k, v) in &fishes_map {
            if *k == 0 {
                new_fishes_map.insert(8, *v);
                new_fishes_map.insert(6, *v + new_fishes_map.get(&6).unwrap_or(&0));
            } else {
                new_fishes_map.insert(k - 1, *v + new_fishes_map.get(&(k - 1)).unwrap_or(&0));
            }
        }

        fishes_map = new_fishes_map;
    }

    fishes_map.values().sum()
}
