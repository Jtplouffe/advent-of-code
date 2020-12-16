use hashbrown::HashMap;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let starting_numbers: Vec<usize> = input.split(",").map(|n| n.parse().unwrap()).collect();

    println!("Part 1: {}", play_slow(&starting_numbers, 2020));
    println!("Part 2: {}", play_fast(&starting_numbers, 30000000));
}

fn play_slow(starting_numbers: &Vec<usize>, until: usize) -> usize {
    let mut numbers = starting_numbers.clone();

    for index in starting_numbers.len()..until {
        numbers.push(
            if let Some(last_index) = (0..index - 1).rev().find(|i| numbers[*i] == numbers[index - 1]) {
                index - (last_index + 1)
            } else {
                0
            }
        );
    }

    *numbers.last().unwrap()
}

fn play_fast(starting_numbers: &Vec<usize>, until: usize) -> usize {
    let mut numbers: HashMap<_, _> = starting_numbers.iter().enumerate().map(|(i, n)| (*n, i)).collect();

    let mut last_number = *starting_numbers.last().unwrap();
    for index in starting_numbers.len()..until {
        let value = if let Some(last_index) = numbers.get(&last_number) {
            index - (last_index + 1)
        } else {
            0
        };

        numbers.insert(last_number, index - 1);
        last_number = value;
    }

    last_number
}