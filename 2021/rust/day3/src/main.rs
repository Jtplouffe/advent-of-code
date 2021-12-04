fn main() {
    let input = include_str!("input");
    let report: Vec<_> = input.lines().collect();

    let energy_consumption = calculate_energy_consumption(&report);
    println!("Part one: {}", energy_consumption);

    let life_support_rating = calculate_life_support_rating(&report);
    println!("Part two: {}", life_support_rating);
}

fn get_most_common_bit_at_index<'a>(report: &Vec<&str>, index: usize, default: &'a str) -> &'a str {
    let mut zero_count = 0;
    for line in report {
        if line.chars().nth(index).unwrap() == '0' {
            zero_count += 1;
        }
    }

    if zero_count > report.len() / 2 {
        "0"
    } else if zero_count < report.len() / 2 {
        "1"
    } else {
        default
    }
}

fn calculate_energy_consumption(report: &Vec<&str>) -> u32 {
    let gamma_rate = u32::from_str_radix(&calculate_gamma_rate(&report), 2).unwrap();
    let mut epsilon_rate = gamma_rate;

    for i in 0..report[0].len() {
        epsilon_rate ^= 1 << i;
    }

    gamma_rate * epsilon_rate
}

fn calculate_gamma_rate(report: &Vec<&str>) -> String {
    let mut dominant_bits = String::new();

    for i in 0..report[0].len() {
        dominant_bits += get_most_common_bit_at_index(report, i, "1");
    }

    dominant_bits
}

fn calculate_life_support_rating(report: &Vec<&str>) -> u32 {
    calculate_oxygen_generator_rating(report) * calculate_co2_scrubber_rating(report)
}

fn calculate_oxygen_generator_rating(report: &Vec<&str>) -> u32 {
    let mut report = report.clone();

    let mut i = 0;
    while report.len() > 1 {
        let most_common_bit = get_most_common_bit_at_index(&report, i, "1");

        for x in (0..report.len()).rev() {
            if report[x].chars().nth(i).unwrap() != most_common_bit.chars().nth(0).unwrap() {
                report.remove(x);
            }
        }

        i += 1;
    }

    u32::from_str_radix(report.first().unwrap(), 2).unwrap()
}

fn calculate_co2_scrubber_rating(report: &Vec<&str>) -> u32 {
    let mut report = report.clone();

    let mut i = 0;
    while report.len() > 1 {
        let most_common_bit = get_most_common_bit_at_index(&report, i, "1");

        for x in (0..report.len()).rev() {
            if report[x].chars().nth(i).unwrap() == most_common_bit.chars().nth(0).unwrap() {
                report.remove(x);
            }
        }

        i += 1;
    }

    u32::from_str_radix(report.first().unwrap(), 2).unwrap()
}