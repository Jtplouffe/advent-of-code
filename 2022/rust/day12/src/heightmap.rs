use std::collections::{HashSet, VecDeque};

type Position = (usize, usize);

pub struct Heightmap {
    heightmap: Vec<Vec<char>>,
    pub start: Position,
    pub destination: Position,
}

impl Heightmap {
    pub fn fewest_steps_to_destination(&self) -> usize {
        self.inner_fewest_steps_to_destination(self.start).unwrap()
    }

    pub fn fewest_steps_to_destination_from_any_start(&self) -> usize {
        let mut step_counts = Vec::new();

        for y in 0..self.heightmap.len() {
            for x in 0..self.heightmap[0].len() {
                if self.heightmap[y][x] == 'a' {
                    if let Some(step_count) = self.inner_fewest_steps_to_destination((x, y)) {
                        step_counts.push(step_count)
                    }
                }
            }
        }

        *step_counts.iter().min().unwrap()
    }

    fn inner_fewest_steps_to_destination(&self, starting_position: Position) -> Option<usize> {
        let mut visited_positions = HashSet::new();
        let mut visitation_queue = VecDeque::new();
        visitation_queue.push_back((starting_position, 0));

        while let Some((current_position, step_count)) = visitation_queue.pop_front() {
            if current_position == self.destination {
                return Some(step_count);
            }

            for adjacent_position in self.adjacent_cells(current_position) {
                if visited_positions.contains(&adjacent_position) {
                    continue;
                }

                if (self.heightmap[current_position.1][current_position.0] as u32 + 1)
                    >= self.heightmap[adjacent_position.1][adjacent_position.0] as u32
                {
                    visited_positions.insert(adjacent_position);
                    visitation_queue.push_back((adjacent_position, step_count + 1));
                }
            }
        }

        None
    }

    fn adjacent_cells(&self, position: Position) -> Vec<Position> {
        let mut positions = Vec::new();

        if position.1 > 0 {
            positions.push((position.0, position.1 - 1));
        }

        if position.0 > 0 {
            positions.push((position.0 - 1, position.1));
        }

        if position.0 < self.heightmap[0].len() - 1 {
            positions.push((position.0 + 1, position.1));
        }

        if position.1 < self.heightmap.len() - 1 {
            positions.push((position.0, position.1 + 1));
        }

        positions
    }
}

impl From<&str> for Heightmap {
    fn from(s: &str) -> Self {
        let heightmap: Vec<Vec<_>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => 'a',
                        'E' => 'z',
                        c => c,
                    })
                    .collect()
            })
            .collect();

        let line_width = s.lines().nth(0).unwrap().len();

        let start_index = s.find('S').unwrap();
        let start = (
            (start_index % (line_width + 1)),
            start_index / (line_width + 1),
        );

        let destination_index = s.find('E').unwrap();
        let destination = (
            (destination_index % (line_width + 1)),
            destination_index / (line_width + 1),
        );

        Self {
            heightmap,
            start,
            destination,
        }
    }
}
