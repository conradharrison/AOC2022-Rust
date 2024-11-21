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
    let mut total_calories_first:i32 = 0;
    let mut total_calories_second:i32 = 0;
    let mut total_calories_third:i32 = 0;
    let mut current_c_count:i32 = 0;
    for c in lines {
        if c == "" {
            // stop adding and check agaist top three
            if current_c_count > total_calories_first {
                total_calories_third = total_calories_second;
                total_calories_second = total_calories_first;
                total_calories_first = current_c_count;
            } else if current_c_count > total_calories_second {
                total_calories_third = total_calories_second;
                total_calories_second = current_c_count;
            } else if current_c_count > total_calories_third {
                total_calories_third = current_c_count;
            }
            current_c_count = 0;
        } else {
            current_c_count += c.parse::<i32>().unwrap();
        }
    }
    // Also do this after last line: stop adding and check agaist max
    if current_c_count > total_calories_first {
        total_calories_third = total_calories_second;
        total_calories_second = total_calories_first;
        total_calories_first = current_c_count;
    } else if current_c_count > total_calories_second {
        total_calories_third = total_calories_second;
        total_calories_second = current_c_count;
    } else if current_c_count > total_calories_third {
        total_calories_third = current_c_count;
    }


    println!("{}", total_calories_first+total_calories_second+total_calories_third);

}
