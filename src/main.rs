use rand::Rng;
use std::fs;
fn main() {
    println!("{}", generate_password("diceware.txt".to_string()))
}
fn generate_password(diceware: String) -> String {
    let mut result: String = String::new();
    let mut iter = 0;
    let contents = fs::read_to_string(diceware).expect("Something went wrong reading the file");
    while iter < 3 {
        result += contents.lines().nth(rand::thread_rng().gen_range(1..1296)).expect("line couldn't be found");
        iter += 1;
    }
    iter = 0;
    while iter < 5 {
        result += &rand::thread_rng().gen_range(1..6).to_string();
        iter += 1;
    }
    result
}