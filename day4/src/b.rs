use std::env;
use std::fs::read_to_string;
use regex::Regex;

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
        let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        let captures = re.captures(l.as_str()).unwrap_or_else(|| panic!("Capture failed for {}", l));
        let s0: u32 = captures[1].parse().unwrap();
        let e0: u32 = captures[2].parse().unwrap();
        let s1: u32 = captures[3].parse().unwrap();
        let e1: u32 = captures[4].parse().unwrap();
        //println!("{} {} {} {}", s0, e0, s1, e1);
        if (e0 >= s1 && e0 <=e1) || (e1 >= s0 && e1 <=e0) {
            //println!("Contained");
            total += 1;
        }
    }

    println!("{total}");
}
