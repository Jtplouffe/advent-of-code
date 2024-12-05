use word_search::WordSearch;

mod word_search;

fn main() {
    let input = include_str!("./input").trim_end();

    let word_search = WordSearch::from(input);

    let xmas_occurrence_count = word_search.count_xmas_occurrence();
    println!("Part 1: {xmas_occurrence_count}");

    let mas_occurrence_count = word_search.count_mas_occurrence();
    println!("Part 2: {mas_occurrence_count}");
}
