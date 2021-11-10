#![feature(str_split_once)]

extern crate ring_algorithm;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let depart_at: u32 = input.lines().nth(0).unwrap().parse().unwrap();
    let busses: Vec<Option<u32>> = input.lines().nth(1).unwrap().split(",").map(|s| {
        if s == "x" {
            None
        } else {
            Some(s.parse().unwrap())
        }
    }).collect();

    println!("Part 1: {}", earliest_bus(depart_at, &busses).and_then(|e| Some(e.0 * e.1)).unwrap());
    println!("Part 2: {}", first_bus_chain_time_fast(&busses));
}

fn earliest_bus(depart_at: u32, busses: &Vec<Option<u32>>) -> Option<(u32, u32)> {
    let mut earliest_bus: Option<(u32, u32)> = None; // (u16 -> bus id, u16 -> time to wait)
    for bus in busses.iter().filter(|b| b.is_some()).map(|b| b.unwrap()) {
        let waiting_time = bus - depart_at % bus;

        if earliest_bus.is_none() || waiting_time < earliest_bus.unwrap().1 {
            earliest_bus = Some((bus, waiting_time));
        }
    }

    earliest_bus
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

// from: https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/13.rs
fn first_bus_chain_time_fast(busses: &Vec<Option<u32>>) -> i64 {
    let busses: Vec<(i64, i64)> = busses.iter().enumerate().filter_map(|(i, b)| b.and_then(|b| Some((i as i64, b as i64)))).collect();

    let mods = busses.iter().map(|&(_,b)| b).collect::<Vec<_>>();
    let res  = busses.iter().map(|&(i,b)| b-i).collect::<Vec<_>>();
    chinese_remainder(&res, &mods).unwrap()
}

// still not finished after 3 hours...
fn first_bus_chain_time_slow(busses: &Vec<Option<u32>>) -> Option<u128> {
    for time in (0 as u128..).step_by(busses.first().unwrap().unwrap() as usize) {
        let mut valid = true;
        for i in 1..(busses.len() as u128) {
            if let Some(bus) = busses[i as usize] {
                if (time + i) % bus as u128 != 0 {
                    valid = false;
                }
            }
        }

        if valid {
            return Some(time);
        }
    }

    None
}