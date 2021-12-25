use crate::seafloor::Seafloor;

mod seafloor;

fn main() {
    let input = include_str!("input");
    let mut seafloor = Seafloor::from(input);

    let mut step = 1;
    while seafloor.step() {
        step += 1;
    }

    println!("Part one: {}", step);
}
