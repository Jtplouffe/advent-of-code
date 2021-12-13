use std::collections::HashMap;

const START: &str = "start";
const END: &str = "end";

pub struct Caves {
    caves: HashMap<String, Vec<String>>,
}

impl Caves {
    pub fn path_to_visit_count_part_1(&self) -> usize {
        self.generate_all_paths_from_path_part_1(vec![START.into()])
            .len()
    }

    pub fn path_to_visit_count_part_2(&self) -> usize {
        self.generate_all_paths_from_path_part_2(vec![START.into()])
            .len()
    }

    fn generate_all_paths_from_path_part_1(&self, path: Vec<String>) -> Vec<Vec<String>> {
        let last_cave = path.last().unwrap();

        if last_cave == END {
            return vec![path];
        }

        let mut paths = Vec::new();

        let connections = self.caves.get(last_cave).unwrap();
        for connection in connections {
            if connection == START
                || connection.to_lowercase().eq(connection) && path.contains(connection)
            {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(connection.to_string());

            for p in self.generate_all_paths_from_path_part_1(new_path) {
                paths.push(p)
            }
        }

        paths
    }

    fn generate_all_paths_from_path_part_2(&self, path: Vec<String>) -> Vec<Vec<String>> {
        let last_cave = path.last().unwrap();

        if last_cave == END {
            return vec![path];
        }

        let mut paths = Vec::new();

        let connections = self.caves.get(last_cave).unwrap();
        for connection in connections {
            if connection == START {
                continue;
            }

            if connection.chars().all(char::is_lowercase) {
                if path.contains(connection) {
                    let max_small_cave_passage = {
                        let mut max = 0;
                        for p in path
                            .iter()
                            .filter(|&p| p.chars().all(char::is_lowercase) && p != START)
                        {
                            let count = path.iter().filter(|&s| s == p).count();
                            if count > max {
                                max = count;
                            }
                        }

                        max
                    };

                    if max_small_cave_passage >= 2 {
                        continue;
                    }
                }
            }

            let mut new_path = path.clone();
            new_path.push(connection.to_string());

            for p in self.generate_all_paths_from_path_part_2(new_path) {
                paths.push(p)
            }
        }

        paths
    }
}

impl From<&str> for Caves {
    fn from(s: &str) -> Self {
        let lines: Vec<_> = s.lines().collect();

        let mut caves = HashMap::<String, Vec<String>>::new();

        for line in lines {
            let (c1, c2) = line.split_once("-").unwrap();

            caves.entry(c1.into()).or_insert(Vec::new()).push(c2.into());
            caves.entry(c2.into()).or_insert(Vec::new()).push(c1.into());
        }

        Self { caves }
    }
}
