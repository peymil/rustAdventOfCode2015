use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let mut seq = content.lines().next().expect("No input").to_owned();
    for _n in 0..40 {
        seq = look_n_say(&seq);
    }
    println!("{}",seq.len());
    for _n in 0..10 {
        seq = look_n_say(&seq);
    }
    println!("{}",seq.len());
}
fn look_n_say(seq: &str) -> String {
    let mut seq_iterator = seq.chars().peekable();
    let mut new_seq = String::new();
    while let Some(chr) = seq_iterator.next(){
        let mut repeat_count:u8 = 1;
        while let Some(next_chr) = seq_iterator.peek(){
            if &chr == next_chr {
                repeat_count += 1;
            }
            else {
                break;
            }
            seq_iterator.next();
        }
        new_seq.push_str(&repeat_count.to_string());
        new_seq.push(chr);
    }
    new_seq
}
