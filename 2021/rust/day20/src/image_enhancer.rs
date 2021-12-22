use crate::Image;

pub struct ImageEnhancer {
    algorithm: Vec<bool>,
}

impl ImageEnhancer {
    pub fn enhance_into(&self, mut image: Image, disable_infinity_pixel: bool, enable_infinite_space: bool) -> Image {
        let mut new_image = Image::new_empty();

        let infinite_pixel = !disable_infinity_pixel && enable_infinite_space;

        let (min_x, min_y, max_x, max_y) = (image.min_x(), image.min_y(), image.max_x(), image.max_y());

        for y in (min_y - 2)..=(max_y + 2) {
            for x in (min_x - 2)..=(max_x + 2) {
                if min_y <= y && y <= max_y && min_x <= x && x <= max_x {
                    continue;
                }

                image.set_pixel(x, y, infinite_pixel);
            }
        }

        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let pixel_square = image.pixel_square(x, y);

                let bin_str = pixel_square.iter().map(|&p| {
                    if p {
                        "1"
                    } else {
                        "0"
                    }
                }).collect::<Vec<_>>().join("");

                let algorithm_index = usize::from_str_radix(&bin_str, 2).unwrap();
                let new_pixel = self.algorithm[algorithm_index];
                new_image.set_pixel(x, y, new_pixel);
            }
        }

        new_image
    }
}

impl From<&str> for ImageEnhancer {
    fn from(s: &str) -> Self {
        let algorithm: Vec<_> = s.chars().map(|c| c == '#').collect();

        Self { algorithm }
    }
}
