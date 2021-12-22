use crate::image::Image;
use crate::image_enhancer::ImageEnhancer;

mod image;
mod image_enhancer;

fn main() {
    let (image_enhancement_algorithm, image) = include_str!("input").split_once("\n\n").unwrap();
    let image_enhancer = ImageEnhancer::from(image_enhancement_algorithm);
    let mut image = Image::from(image);

    let disable_infinity_pixel = image_enhancement_algorithm.chars().nth(0).unwrap() == '.';

    for i in 0..50 {
        image = image_enhancer.enhance_into(image, disable_infinity_pixel, i % 2 == 1);

        if i == 1 {
            println!("Part one: {}", image.pixel_count());
        }
    }

    println!("Part one: {}", image.pixel_count());
}
