use std::collections::{HashMap, VecDeque};

use crate::command::Command;
use crate::file_system::directory::Directory;
use crate::file_system::file::File;

pub mod directory;
pub mod file;

#[derive(Debug)]
pub struct FileSystem<'a> {
    directories: HashMap<String, Directory<'a>>,
}

impl<'a> FileSystem<'a> {
    pub fn recreate_from_commands(commands: &'a [Command]) -> Self {
        let mut directories = HashMap::new();

        let mut current_path = VecDeque::new();
        for command in commands {
            match command {
                Command::ChangeDirectory(cd) => {
                    match cd.directory {
                        ".." => {
                            current_path.pop_front();
                        },
                        "/" => {
                            current_path.clear();
                            current_path.push_front("/");
                        },
                        directory => {
                            current_path.push_front(directory);
                        }
                    }
                }
                Command::List(ls) => {
                    let mut files = Vec::new();

                    for output_line in ls.output.lines() {
                        if output_line.starts_with("dir") {
                            // We can probably ignore directory, since we create them on navigation
                            continue;
                        }

                        let (size, name) = output_line.split_once(' ').unwrap();
                        files.push(File::new(name, size.parse().unwrap()))
                    }

                    let mut path = String::new();
                    for &p in current_path.iter().rev() {
                        if p != "/" && !path.ends_with('/') {
                            path.push('/');
                        }

                        path += p;
                    }

                    directories.insert(path, Directory::new(current_path.get(0).unwrap(), files));
                }
            }
        }

        Self { directories }
    }

    pub fn deletion_candidates_total_size(&self) -> usize {
        self.get_directories_size().values().filter(|&size| *size <= 100000).sum()
    }

    pub fn size_of_directory_to_be_deleted(&self) -> usize {
        let directories_size = self.get_directories_size();
        let total_used_space = *directories_size.get("/").unwrap();
        let space_needed = 30_000_000 - (70_000_000 - total_used_space);

        *directories_size.values().filter(|&value| *value >= space_needed).min().unwrap()
    }

    fn get_directories_size(&self) -> HashMap<String, usize> {
        let mut directories_size = HashMap::<String, usize>::new();

        for path in self.directories.keys() {
            let mut current_path = path.clone();

            while current_path.len() > 0 {
                if directories_size.contains_key(&*current_path) {
                    break;
                }

                let size = self.get_directory_total_size(&current_path);
                directories_size.insert(current_path.to_string(), size);

                while !current_path.ends_with('/') {
                    current_path.pop();
                }
                current_path.pop();
            }
        }

        directories_size
    }

    fn get_directory_total_size(&self, path: &str) -> usize {
        let mut sum = 0;

        for p in self.directories.keys() {
            if p.starts_with(path) {
                sum += self.directories[p].total_size();
            }
        }

        sum
    }
}