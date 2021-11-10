use crate::waiting_room::WaitingRoom;

mod waiting_room;
mod place;
mod direction;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let mut waiting_room = WaitingRoom::from(input);

    while waiting_room.run_part_one() {}
    println!("Part 1: {}", waiting_room.count_occupied());

    waiting_room.reset();

    while waiting_room.run_part_two() {}
    println!("Part 2: {}", waiting_room.count_occupied());
}
