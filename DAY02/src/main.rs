use std::env;
use std::fs;
extern crate regex;

fn main() {
    let re = regex::Regex::new(r"x").unwrap();
    let contents =
        fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file");
    let mut paper_result: u32 = 0;
    let mut ribbon_resut: u32 = 0;

    for a in contents.lines().into_iter() {
        let mut d: Vec<u32> = re
            .split(a)
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        d.sort();
        ribbon_resut += d[0] * 2 + d[1] * 2 + d[0] * d[1] * d[2];
        paper_result += 2 * (d[0] * d[1]) + 2 * (d[1] * d[2]) + 2 * (d[0] * d[2]) + d[0] * d[1];
    }
    println!("{} feet wrapping paper needed.", paper_result);
    println!("{} feet ribbon  needed.", ribbon_resut);
}
