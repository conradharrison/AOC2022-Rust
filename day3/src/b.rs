use std::env;
use std::fs::read_to_string;
use itertools::Itertools;

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

fn char_to_priority(c: &char) -> u32 {
    let ascii = *c as u32;
    if ascii <= 90 {
        ascii - 65 + 27
    } else {
        ascii - 97 + 1
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
    let mut total: u32 = 0;
    for (l1, l2, l3) in lines.iter().tuples() {

        // Sort by size of lines
        let s1: &String;
        let s2: &String;
        let s3: &String;

        if l1.len() >= l2.len() && l1.len() >= l3.len() {
            s1 = &l1;
            if l2.len() >= l1.len() {
                s2 = &l2;
                s3 = &l3;
            } else {
                s2 = &l3;
                s3 = &l2;
            }
        } else if l2.len() >= l1.len() && l2.len() >= l3.len() {
            s1 = &l2;
            if l1.len() >= l3.len() {
                s2 = &l1;
                s3 = &l3;
            } else {
                s2 = &l3;
                s3 = &l1;
            }
        } else {
            s1 = &l3;
            if l2.len() >= l1.len() {
                s2 = &l2;
                s3 = &l1;
            } else {
                s2 = &l1;
                s3 = &l2;
            }
        }

        //println!("{:?}", s1);
        //println!("{:?}", s2);
        //println!("{:?}", s3);

        for c in s3.chars() {
            //println!("Looking for {c} in {s2}");
            if s2.find(c) == None {
                continue;
            } else {
                //println!("Looking for {c} in {s1}");
                if s1.find(c) == None {
                    continue;
                } else {
                    println!("Found {} with priority {}", c, char_to_priority(&c));
                    total += char_to_priority(&c);
                    break;
                }
            }
        }

    }

    println!("{total}");
}
