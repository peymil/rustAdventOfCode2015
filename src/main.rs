// "(" one floor up
// ")" one floor down
use std::env;
use std::fs;
fn main() {
    let contents = fs::read_to_string("./src/input.txt")
    .expect("Something went wrong reading the file");
    contents.chars().into_iter().map(|x| println!("{}",x));
}
