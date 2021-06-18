use rand::Rng;
use std::{env, fs};
fn main() {
    println!("{}", generate_password("diceware.txt".to_string()))
}
fn generate_password(diceware: String) -> String {
    let args: Vec<String> = env::args().collect();
    let mut result: String = String::new();
    let mut iter = 0;
    let word_count: i32 = if args.len() < 2 {
        3
    } else {
        args[1].parse::<i32>().unwrap()
    };
    let num_count: i32 = if args.len() < 3 {
        5
    } else {
        args[2].parse::<i32>().unwrap()
    };
    let contents = fs::read_to_string(diceware).expect("Something went wrong reading the file");
    while iter < word_count {
        result += contents
            .lines()
            .nth(rand::thread_rng().gen_range(1..7776))
            .expect("line couldn't be found");
        iter += 1;
        if iter == 1 {
            result = result.to_ascii_uppercase();
        }
    }
    iter = 0;
    while iter < num_count {
        result += &rand::thread_rng().gen_range(1..10).to_string();
        iter += 1;
    }
    result
}
