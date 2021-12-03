mod lib;

use lib::{function_mapping, input_helpers};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let [day, puzzle, file_path] = &args[1..] {
        let day: u32 = day.parse().expect("Day argument must be a number");
        let puzzle: u32 = puzzle.parse().expect("Puzzle argument must be a number");

        let puzzle_function = function_mapping::get_puzzle_function(day, puzzle);

        if let Some(puzzle_function) = puzzle_function {
            let file_contents = input_helpers::get_contents_of_file(file_path);
            match puzzle_function(file_contents) {
                Ok(return_value) => println!("{}", return_value),
                Err(error) => println!("ERROR: Failed to get result:\n{}", error),
            }
        } else {
            println!(
                "No matching function found for day: {}, puzzle: {}",
                day, puzzle
            );
        }
    } else {
        println!("Usage: {} <day> <puzzle> <input file path>", &args[0]);
    }
}
