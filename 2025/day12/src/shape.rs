pub struct Shape {
    pub index: usize,
    pub shape: Vec<Vec<bool>>,
}

impl Shape {
    pub fn width(&self) -> usize {
        self.shape.first().map(|line| line.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.shape.len()
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let start = lines.next().unwrap();
        let index = start[..start.len() - 1].parse().expect("Invalid index");

        let mut shape = Vec::new();
        for line in lines {
            let mut shape_line = Vec::new();
            for char in line.chars() {
                match char {
                    '.' => shape_line.push(false),
                    '#' => shape_line.push(true),
                    _ => unreachable!(),
                }
            }

            shape.push(shape_line);
        }

        Self { index, shape }
    }
}
