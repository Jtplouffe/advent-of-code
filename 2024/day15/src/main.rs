use warehouse::Warehouse;

mod position;
mod robot;
mod tile;
mod warehouse;

fn main() {
    let input = include_str!("./input");

    let mut first_warehouse = Warehouse::from(input);
    let mut second_warehouse = first_warehouse.clone();

    first_warehouse.perform_robot_movements();

    println!("Part 1: {}", first_warehouse.box_gps_coordinate_sum());

    second_warehouse.double_width();

    second_warehouse.perform_robot_movements();
    println!("Part 2: {}", second_warehouse.box_gps_coordinate_sum());
}
