use change_directory_command::ChangeDirectoryCommand;
use list_command::ListCommand;
use crate::command::Command::{ChangeDirectory, List};

pub mod list_command;
pub mod change_directory_command;

pub enum Command<'a> {
    ChangeDirectory(ChangeDirectoryCommand<'a>),
    List(ListCommand),
}

impl<'a> Command<'a> {
    pub fn from_input(s: &str) -> Vec<Command> {
        let mut commands = Vec::new();

        let lines = s.lines().collect::<Vec<_>>();

        let mut i = 0;
        while i < lines.len() {
            assert!(lines[i].starts_with('$'));

            let mut split = lines[i].split(' ');
            split.next(); // Skip the '$'

            let command = match split.next().unwrap() {
                "cd" => ChangeDirectory(ChangeDirectoryCommand::new(split.next().unwrap())),
                "ls" => {
                    let mut output = Vec::new();

                    while i + 1 < lines.len() && !lines[i + 1].starts_with('$') {
                        output.push(lines[i + 1]);
                        i += 1;
                    }

                    List(ListCommand::new(output.join("\n")))
                },
                a => unreachable!("Reached {}", a),
            };

            commands.push(command);
            i += 1;
        }

        commands
    }
}