use crate::boarding_pass::BoardingPass;

mod boarding_pass;

fn main() {
    let input = std::fs::read_to_string("assets/input").unwrap();
    let raw_boarding_passes = input.split("\n");

    let mut boarding_passes: Vec<BoardingPass> = raw_boarding_passes.map(|b| b.parse::<BoardingPass>().unwrap()).collect();
    boarding_passes.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    println!("Part 1: {}", boarding_passes.iter().map(|b| b.id).max().unwrap());

    let mut boarding_pass_id = 0;
    for i in 0..boarding_passes.len() {
        if let Some(next) = boarding_passes.get(i + 1) {
            let current = boarding_passes.get(i).unwrap();
            if next.id == current.id + 2 {
                boarding_pass_id = current.id + 1;
                break;
            }
        }
    }

    println!("Part 2: {}", boarding_pass_id);
}
