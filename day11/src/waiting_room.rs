use crate::direction::Direction;
use crate::place::Place;

pub struct WaitingRoom {
    pub places: Vec<Vec<Place>>,
}

impl WaitingRoom {
    pub fn run_part_one(&mut self) -> bool {
        let mut changed = false;

        let mut new_places = self.places.clone();

        for y in 0..self.places.len() {
            for x in 0..self.places[y].len() {
                match self.places[y][x] {
                    Place::Empty => {
                        if self.count_adjacent(y as isize, x as isize, Place::Occupied) == 0 {
                            new_places[y][x] = Place::Occupied;
                            changed = true;
                        }
                    }
                    Place::Occupied => {
                        if self.count_adjacent(y as isize, x as isize, Place::Occupied) >= 4 {
                            new_places[y][x] = Place::Empty;
                            changed = true;
                        }
                    }
                    Place::Floor => new_places[y][x] = Place::Floor,
                }
            }
        }

        self.places = new_places;
        changed
    }

    pub fn run_part_two(&mut self) -> bool {
        let mut changed = false;

        let mut new_places = self.places.clone();

        for y in 0..self.places.len() {
            for x in 0..self.places[y].len() {
                match self.places[y][x] {
                    Place::Empty => {
                        if self.count_all_directions(y as isize, x as isize, Place::Occupied) == 0 {
                            new_places[y][x] = Place::Occupied;
                            changed = true;
                        }
                    }
                    Place::Occupied => {
                        if self.count_all_directions(y as isize, x as isize, Place::Occupied) >= 5 {
                            new_places[y][x] = Place::Empty;
                            changed = true;
                        }
                    }
                    Place::Floor => new_places[y][x] = Place::Floor,
                }
            }
        }

        self.places = new_places;
        changed
    }

    fn count_adjacent(&self, y: isize, x: isize, place_type: Place) -> u8 {
        let mut count = 0;

        for yn in y - 1..=y + 1 {
            for xn in x - 1..=x + 1 {
                if let Some(place) = self.get_place(yn, xn) {
                    if *place == place_type && !(yn == y && xn == x) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn count_all_directions(&self, y: isize, x: isize, place_type: Place) -> u8 {
        let mut count = 0;

        for dir in Direction::values() {
            let delta = dir.value();
            let (mut yn, mut xn) = (y + delta.1, x + delta.0);

            while let Some(place) = self.get_place(yn, xn) {
                if *place != Place::Floor {
                    if *place == place_type {
                        count += 1;
                    }

                    break;
                }
                yn += delta.1;
                xn += delta.0;
            }
        }

        count
    }

    fn get_place(&self, y: isize, x: isize) -> Option<&Place> {
        if y < 0 || x < 0 {
            return None;
        }

        self.places.get(y as usize).and_then(|line| line.get(x as usize))
    }

    pub fn count_occupied(&self) -> usize {
        self.places
            .iter()
            .flatten()
            .filter(|&p| *p == Place::Occupied)
            .count()
    }

    pub fn reset(&mut self) {
        self.places = self.places.iter().map(|line| {
            line.iter().map(|place| {
                if *place == Place::Occupied {
                    Place::Empty
                } else {
                    place.clone()
                }
            }).collect()
        }).collect();
    }
}

impl From<String> for WaitingRoom {
    fn from(s: String) -> Self {
        Self {
            places: s
                .lines()
                .map(|l| l.chars().map(|c| c.into()).collect())
                .collect(),
        }
    }
}
