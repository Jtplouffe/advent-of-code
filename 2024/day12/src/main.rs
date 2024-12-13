use field::Field;

mod field;
mod position;
mod garden;

fn main() {
    let input = include_str!("./input").trim_end();

    let field = Field::from(input);

    let total_fence_price = field.total_fence_price();
    println!("Part 1: {total_fence_price}");

    let total_fence_bulk_price = field.total_fence_bulk_price();
    println!("Part 2: {total_fence_bulk_price}");
}
