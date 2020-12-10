fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let invalid_number = first_invalid_number(&numbers).unwrap();
    println!("Part 1: {}", invalid_number);
    println!("Part 2: {}", find_weakness(&numbers, invalid_number).unwrap());
}

fn first_invalid_number(numbers: &Vec<u64>) -> Option<u64> {
    for i in 25..numbers.len() {
        if !can_sum_to(&numbers[i - 25..i], numbers[i]) {
            return Some(numbers[i]);
        }
    }
    None
}

fn can_sum_to(numbers: &[u64], value: u64) -> bool {
    for n in 0..numbers.len() {
        for m in n..numbers.len() {
            if value == numbers[n] + numbers[m] {
                return true;
            }
        }
    }
    false
}

fn find_weakness(numbers: &Vec<u64>, invalid_number: u64) -> Option<u64> {
    if let Some((first, last)) = find_weakness_range(numbers, invalid_number) {
        let min = numbers[first..=last].iter().min().unwrap();
        let max = numbers[first..=last].iter().max().unwrap();
        return Some(min + max);
    }
    None
}

fn find_weakness_range(numbers: &Vec<u64>, invalid_number: u64) -> Option<(usize, usize)> {
    for i in 0..numbers.len() {
        let mut value = 0;
        let mut offset = 0;
        while value < invalid_number && i + offset < numbers.len() {
            value += numbers[i + offset];
            offset += 1;
        }
        offset -= 1;

        if offset >= 2 && value == invalid_number {
            return Some((i, i + offset));
        }
    }
    None
}