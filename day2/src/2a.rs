use std::env;
use std::fs::read_to_string;

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

#[derive(Debug)]
enum GameResult {
    WIN = 6,
    LOSE = 0,
    DRAW = 3
}

#[derive(Debug, Clone, PartialEq)]
enum Hand {
    ROCK = 0,
    PAPER = 1,
    SCISSOR = 2
}

impl Hand {
    fn challenge(&self, h: &Hand) -> GameResult {
        let x: i8 = unsafe { *(self as *const Self as *const i8) };
        let y: i8 = unsafe { *(h as *const Self as *const i8) };
        let winner: i8 = (x + 1).rem_euclid(3);
        let loser: i8 = (x - 1).rem_euclid(3);

        let r: GameResult;
        if (y as i8) == winner { r = GameResult::LOSE }
        else if (y as i8) == loser { r = GameResult::WIN }
        else { r = GameResult::DRAW }
        
        // Debug
        //println!("{self:?} {r:?} {h:?}");
        
        return r;
    }

    fn challenge2(&self, h: Hand) -> GameResult {
        if h == self.winner() { return GameResult::LOSE; }
        else if h == self.loser() { return GameResult::WIN; }
        else if h == *self { return GameResult::DRAW; }
        else { panic!("Illegal hand found: {h:?}") }
    }

    fn points(&self) -> u8 {
        let x: u8 = unsafe { *(self as *const Self as *const u8) };
        return x + 1;
    }

    fn winner(&self) -> Hand {
        match self {
            Hand::ROCK    => { return Hand::PAPER },
            Hand::PAPER   => { return Hand::SCISSOR },
            Hand::SCISSOR => { return Hand::ROCK },
        }
    }

    fn loser(&self) -> Hand {
        match self {
            Hand::ROCK    => { return Hand::SCISSOR },
            Hand::PAPER   => { return Hand::ROCK },
            Hand::SCISSOR => { return Hand::PAPER },
        }
    }

}

fn theirs_to_hand(s: char) -> Hand {
    match s {
        'A' => { return Hand::ROCK },
        'B' => { return Hand::PAPER },
        'C' => { return Hand::SCISSOR },
          _ => { panic!("Illegal hand found: {s:?}") }
    }
}

fn ours_to_hand(s: char) -> Hand {
    match s {
        'X' => { return Hand::ROCK },
        'Y' => { return Hand::PAPER },
        'Z' => { return Hand::SCISSOR },
          _ => { panic!("Illegal hand found: {s:?}") }
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
    let mut total_points: i32 = 0;
    for c in lines {
        let theirs: Hand = theirs_to_hand(c.chars().nth(0).unwrap());
        let ours: Hand = ours_to_hand(c.chars().nth(2).unwrap());
       
        // One or the other
        //total_points += ours.challenge(&theirs) as i32;
        total_points += ours.challenge2(theirs) as i32;
        
        total_points += i32::from(ours.points());
    }

    println!("{total_points}");
}
