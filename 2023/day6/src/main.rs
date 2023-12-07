use race::Race;

mod race;

fn main() {
    let input = include_str!("./input");

    let mut lines = input.lines();
    let short_times: Vec<_> = extract_numbers(lines.next().expect("expected line")).collect();
    let short_record_distances: Vec<_> =
        extract_numbers(lines.next().expect("expected line")).collect();

    let long_time = short_times
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    let long_record_distance = short_record_distances
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    let short_races: Vec<_> = short_times
        .into_iter()
        .zip(short_record_distances)
        .map(|(time, record_distance)| Race::new(time, record_distance))
        .collect();

    let long_race = Race::new(long_time, long_record_distance);

    let short_possible_ways_to_beat_records_product: usize = short_races
        .iter()
        .map(Race::possible_ways_to_beat_record)
        .product();
    println!("Part 1: {short_possible_ways_to_beat_records_product}");

    let long_possible_ways_to_beat_record = long_race.possible_ways_to_beat_record();
    println!("Part 2: {long_possible_ways_to_beat_record}");
}

fn extract_numbers(line: &str) -> impl Iterator<Item = usize> + '_ {
    line.split(':')
        .nth(1)
        .expect("expected type delimiter")
        .split_whitespace()
        .filter_map(|number| {
            if number.is_empty() {
                None
            } else {
                number.parse::<usize>().ok()
            }
        })
}
