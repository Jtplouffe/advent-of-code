use crate::ingredient_database::IngredientDatabase;

mod ingredient_database;

fn main() {
    let input = include_str!("./input").trim();
    let ingredient_database = IngredientDatabase::from(input);

    println!("Part 1: {}", ingredient_database.fresh_ingredient_count());
    println!(
        "Part 2: {}",
        ingredient_database.total_possible_fresh_ingredient_count()
    );
}
