use std::env;
use std::fs::read_to_string;

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

fn main() {

    // Read in data file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must provide data file path (only)");
    }
    let file_path = &args[1];

    let lines = read_lines(file_path);

    println!("Read the following from {file_path}:\n{lines:?}");

    // Solution
    let mut max_calories:i32 = 0;
    let mut current_c_count:i32 = 0;
    for c in lines {
        if c == "" {
            // stop adding and check agaist max
            if max_calories < current_c_count {
                max_calories = current_c_count;
            }
            current_c_count = 0;
        } else {
            current_c_count += c.parse::<i32>().unwrap();
        }
    }
    // Also do this after last line: stop adding and check agaist max
    if max_calories < current_c_count {
        max_calories = current_c_count;
    }

    println!("{max_calories}");

}
