// "(" one floor up
// ")" one floor down
use std::env;
use std::fs;
fn main() {
    let mut number = 0;
    let mut position = 0;
    let mut didEnteredToBasement = false;
    let contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    for x in contents.chars().into_iter() {
        position += 1;

        match x {
            '(' => number += 1,
            ')' => number -= 1,
            _ => println!("{}", "exception"),
        }
        if (!didEnteredToBasement && number == -1) {
            didEnteredToBasement = true;
            print!("{}", "Santa entered to basement at ");
            println!("{}", position);
        }
    }
    println!("{}{}", "Santa found the floor ", number)
}
