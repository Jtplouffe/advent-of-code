use crate::transparent_sheet::TransparentSheet;

mod fold;
mod orientation;
mod transparent_sheet;

fn main() {
    let input = include_str!("input");
    let mut transparent_sheet = TransparentSheet::from(input);

    transparent_sheet.fold_once();
    println!("Part one: {}", transparent_sheet.dot_count());

    transparent_sheet.fold_rest();
    println!("Part two:\n{}", transparent_sheet);
}
