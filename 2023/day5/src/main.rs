use almanac::Almanac;

mod almanac;
mod category_converter;
mod category_converter_range;

fn main() {
    let input = include_str!("./input");

    let almanac = Almanac::from(input);

    let lowest_location = almanac.lowest_location();
    println!("Part 1: {lowest_location}");

    let lowest_location_from_seed_range = almanac.lowest_location_from_seed_ranges();
    println!("Part 2: {lowest_location_from_seed_range}");
}
