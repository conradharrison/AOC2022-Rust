use std::env;
use std::fs::read_to_string;
use regex::Regex;
use std::mem;

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

fn construct_stack_list(line_stack: &mut Vec<String>, n: usize) -> Vec<Vec<char>> {
    // Create a Vec of Vec<char>
    let mut stack_list: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    for _i in 0..n {
        stack_list.push(Vec::<char>::new());
    }
    
    //println!("{:?}", stack_list);
    //println!("{:?}", line_stack);

    while line_stack.len() > 0 {
        let to_push = line_stack.pop().unwrap();
        //println!("{:?}", to_push);
        for i in 0..n {
        //println!("{:?}, {:?}", to_push.chars().nth(i*4).unwrap(), to_push.chars().nth(i*4+1).unwrap());
            if to_push.chars().nth(i*4).unwrap() == '[' {
                //println!("Pushed");
                stack_list.get_mut(i).unwrap().push(to_push.chars().nth(i*4+1).unwrap());
            } else {
                continue;
            }
        }
    }
    return stack_list;
}

fn do_move(stack_list: &mut Vec<Vec<char>>, count: &str, from: &str, to: &str) {

    //let from_stack: &mut Vec<char> =  &mut(stack_list[from.parse::<usize>().unwrap()]);
    //let to_stack: &mut Vec<char> = &mut(stack_list[to.parse::<usize>().unwrap()]);
    
    let count = count.parse::<usize>().unwrap();
    let from = from.parse::<usize>().unwrap() - 1;
    let to = to.parse::<usize>().unwrap() - 1;
    
    //println!("stack_list is: {stack_list:?}");

    let mut from_stack = mem::take(&mut stack_list[from]);
    let mut to_stack = mem::take(&mut stack_list[to]);
    
    //println!("Moving {count} from {} to {}", from+1, to+1);
    //println!("from stack: {from_stack:?}");
    //println!("to stack: {to_stack:?}");

    for _i in 0..count {
        to_stack.push(from_stack.pop().unwrap());
    }

    //println!("from stack is now: {from_stack:?}");
    //println!("to stack is now: {to_stack:?}");
    //println!("stack_list is now: {stack_list:?}");

    stack_list[from] = from_stack;
    stack_list[to] = to_stack;

    //println!("from stack is now back to: {stack_list:?}");
    //println!("");
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
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut line_stack: Vec<String> = Vec::<String>::new();
    let mut stack_list: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    let mut top_list: Vec<char> = Vec::<char>::new();

    let mut found_stacks: bool = false;
    for l in lines {
        
        if !found_stacks {
            if l.chars().nth(1).unwrap() == '1' {
                found_stacks = true;
                //println!("Count line = {l:?}");
                stack_list = construct_stack_list(&mut line_stack, l.split_whitespace().count());
            } else {
                line_stack.push(l);
            }
        } else {
                let captures = re.captures(l.as_str());
                match captures {
                    None => { continue; },
                    Some(c) => {
                        let count = c.get(1).unwrap().as_str();
                        let from = c.get(2).unwrap().as_str();
                        let to = c.get(3).unwrap().as_str();
                        do_move(&mut stack_list, &count, &from, &to);
                    }
                }
        }
    }

    for stack in stack_list {
        match stack.last() {
            None => {},
            Some(&c) => { top_list.push(c); }
        }
    }

    println!("{:?}", top_list.iter().collect::<String>());
}
