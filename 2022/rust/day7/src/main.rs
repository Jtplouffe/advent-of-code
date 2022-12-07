extern crate core;

use std::time::Instant;
use crate::command::Command;
use crate::file_system::FileSystem;

mod file_system;
mod command;

fn main() {
    let i = Instant::now();
    let input = include_str!("./input");

    let commands = Command::from_input(input);
    let file_system = FileSystem::recreate_from_commands(&commands);

    let deletion_candidates_total_size = file_system.deletion_candidates_total_size();
    println!("Part 1: {deletion_candidates_total_size }");

    let size_of_directory_to_be_deleted = file_system.size_of_directory_to_be_deleted();
    println!("Part 2: {size_of_directory_to_be_deleted}");

    println!("{:?}", i.elapsed());
}
