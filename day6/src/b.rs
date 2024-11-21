use std::env;
use std::fs::read_to_string;

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

fn check_if_last_n_is_unique(n: usize, slice: &str) -> bool {
    //println!("{slice} {n}");
    if n == 1 {
        return true;
    } else {
        let mut i_am_unique: bool = true;
        for i in 0..n-1 {
            //if slice.get(i).unwrap() == slice.get(n).unwrap() {
            if slice[i..i+1] == slice[n-1..n] {
                i_am_unique = false;
                break;
            }
        }
        return check_if_last_n_is_unique(n-1, &slice[0..n-1]) && i_am_unique; 
    }
}

fn main() {

    // Read in data file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must provide data file path (only)");
    }
    let file_path = &args[1];

    let lines = read_lines(file_path);
    
    // Debug
    //println!("Read the following from {file_path}:\n{lines:?}");

    // Solution
    let window_size = 14;
    for line in lines {
        for ci in (window_size-1)..line.len() {
            let c = line.chars().nth(ci).unwrap();
            let unique = check_if_last_n_is_unique(window_size, line.get(ci-(window_size-1)..ci+1).unwrap());
            if unique {
                println!("{} at {}", c, ci+1);
                break;
            }
        }
    }
}
