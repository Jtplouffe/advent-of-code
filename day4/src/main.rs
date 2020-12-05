use crate::passport::Passport;

mod passport;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let raw_passports = input.split("\n\n");

    let passports: Vec<Passport> = raw_passports.map(|p| p.parse::<Passport>().unwrap()).collect();

    println!("Part 1: {}", passports.iter().filter(|p| p.has_all_required_fields()).count());
    println!("Part 2: {}", passports.iter().filter(|p| p.is_valid()).count());
}