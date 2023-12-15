use platform::Platform;

mod rock_type;
mod platform;

fn main() {
    let input = include_str!("./input");

    let mut platform_part_1 = Platform::from(input);
    let mut platform_part_2 = platform_part_1.clone();

    platform_part_1.tilt_north();
    println!("Part 1: {}", platform_part_1.north_support_beam_load());

    platform_part_2.rotate(1_000_000_000);
    println!("Part 2: {}", platform_part_2.north_support_beam_load());
}
