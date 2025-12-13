pub struct Region {
    pub width: usize,
    pub height: usize,
    pub shapes_quantity: Vec<usize>,
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (size, shapes_quantity) = value.split_once(": ").expect("Invalid region");

        let (width, height) = size.split_once('x').expect("Invalid size");
        let width = width.parse().expect("Invalid width");
        let height = height.parse().expect("Invalid height");

        let shapes_quantity = shapes_quantity
            .split(' ')
            .map(|count| count.parse().expect("Invaild quantity"))
            .collect();

        Self {
            width,
            height,
            shapes_quantity,
        }
    }
}
