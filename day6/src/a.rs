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
    
    // Debug
    //println!("Read the following from {file_path}:\n{lines:?}");

    // Solution
    for line in lines {
        let mut w0 = line.chars().nth(0).unwrap();
        let mut w1 = line.chars().nth(1).unwrap();
        let mut w2 = line.chars().nth(2).unwrap();
        for ci in 3..line.len() {
            let c = line.chars().nth(ci).unwrap();
            if c != w0  && c != w1 && c != w2 && w2 != w1 && w2 != w0 && w1 != w0 {
                println!("{} at {}", c, ci+1);
                break;
            } else {
                w0 = w1;
                w1 = w2;
                w2 = c;
            }
        }
    }
}
