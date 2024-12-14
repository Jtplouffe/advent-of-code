use room::Room;

mod position;
mod robot;
mod room;

const ROOM_WIDTH: usize = 101;
const ROOM_HEIGHT: usize = 103;

fn main() {
    let input = include_str!("./input").trim_end();

    let mut room1 = Room::new(ROOM_WIDTH, ROOM_HEIGHT, input);
    let mut room2 = room1.clone();

    room1.process(100);
    println!("Part 1: {}", room1.safety_factory());

    let part2 = room2.seconds_for_first_christmas_tree();
    println!("Part 2: {part2}");
}
