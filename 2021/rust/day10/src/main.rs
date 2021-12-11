use crate::program::Program;

mod instruction;
mod program;
mod program_line;

fn main() {
    let input = include_str!("input");
    let program = Program::from(input);

    println!("Part one: {}", program.get_syntax_error_score());
    println!("Part two: {}", program.autocomplete_score());
}
