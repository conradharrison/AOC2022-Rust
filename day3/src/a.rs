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
    let mut total: u32 = 0;
    for l in lines {
        let middle = l.len()/2;
        let first_half = &(l.as_str()[..middle]);
        let second_half = &(l.as_str()[middle..]);
        for c in first_half.chars() {
            let r = second_half.find(c);
            if r != None {
                let ascii = c as u32;
                let priority: u32;
                if ascii <= 90 {
                    priority = ascii - 65 + 27;
                } else {
                    priority = ascii - 97 + 1;
                }
                total += priority;
                println!("{:?} {:?} {:?}", c, priority, total);
                break;
            }
        }
    }

    println!("{total}");
}
