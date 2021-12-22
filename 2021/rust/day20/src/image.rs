use std::collections::HashSet;

#[derive(Debug)]
pub struct Image {
    pixels: HashSet<(isize, isize)>,
}

impl Image {
    pub fn new_empty() -> Self {
        Self { pixels: HashSet::new() }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, value: bool) {
        if value {
            self.pixels.insert((x, y));
        } else {
            self.pixels.remove(&(x, y));
        }
    }

    pub fn pixel_at(&self, x: isize, y: isize) -> bool {
        self.pixels.contains(&(x, y))
    }

    pub fn pixel_square(&self, x: isize, y: isize) -> Vec<bool> {
        let mut pixels = Vec::with_capacity(9);

        for y in (y - 1)..=(y + 1) {
            for x in (x - 1)..=(x + 1) {
                pixels.push(self.pixel_at(x, y));
            }
        }

        pixels
    }

    pub fn min_x(&self) -> isize {
        *self.pixels.iter().map(|(x, _)| x).min().unwrap()
    }

    pub fn min_y(&self) -> isize {
        *self.pixels.iter().map(|(_, y)| y).min().unwrap()
    }

    pub fn max_x(&self) -> isize {
        *self.pixels.iter().map(|(x, _)| x).max().unwrap()
    }

    pub fn max_y(&self) -> isize {
        *self.pixels.iter().map(|(_, y)| y).max().unwrap()
    }

    pub fn pixel_count(&self) -> usize {
        self.pixels.len()
    }
}

impl From<&str> for Image {
    fn from(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();

        let mut pixels = HashSet::with_capacity(s.len());
        for (y, &line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    pixels.insert((x as isize, y as isize));
                }
            }
        }

        Self { pixels }
    }
}
