#![feature(str_split_once)]

use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let raw_blocks = input.replace("mem[", "").replace("] =", "");
    let program = raw_blocks.split("mask = ").filter(|s| *s != "").collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&program));
    println!("Part 2: {}", part_two(&program));
}

fn part_one(program: &Vec<&str>) -> usize {
    let mut memory = HashMap::<usize, usize>::new();

    for block in program {
        let lines = block.lines().collect::<Vec<_>>();
        let mask = usize::from_str_radix(&lines.first().unwrap().replace("X", "0"), 2).unwrap();
        let block_mask = !usize::from_str_radix(
            &lines.first().unwrap().replace("1", "X").replace("0", "1").replace("X", "0"),
            2,
        ).unwrap();

        for line in &lines[1..] {
            let (addr, value): (usize, usize) = line.split_once(" ").and_then(|(m, v)| Some((m.parse().unwrap(), v.parse().unwrap()))).unwrap();
            memory.insert(addr, (value | mask) & block_mask);
        }
    }

    memory.iter().map(|(_, v)| v).sum()
}

fn part_two(program: &Vec<&str>) -> usize {
    let mut memory = HashMap::<usize, usize>::new();

    for block in program {
        let lines = block.lines().collect::<Vec<_>>();
        let floating_bit_indexes = lines.first().unwrap().chars().rev().enumerate().filter_map(|(i, c)| {
            if c == 'X' {
                Some(i)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        let raw_mask = lines.first().unwrap();
        let mask = usize::from_str_radix(&raw_mask.replace("X", "0"), 2).unwrap();

        for line in &lines[1..] {
            let (raw_addr, value): (usize, usize) = line.split_once(" ").and_then(|(m, v)| Some((m.parse().unwrap(), v.parse().unwrap()))).unwrap();
            let mut addresses = vec![raw_addr & !usize::from_str_radix(&raw_mask.replace("1", "0").replace("X", "1"), 2).unwrap() | mask];
            memory.insert(addresses[0], value);

            for i in &floating_bit_indexes {
                let addr = 1 << i;

                for a in addresses.clone() {
                    addresses.push(a | addr);
                    memory.insert(a | addr, value);
                }
            }
        }
    }

    memory.iter().map(|(_, v)| v).sum()
}