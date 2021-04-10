use md5;
fn main() {
    let input = "bgvyzdsv";
    println!("Answer of first question: {:?}", solve(input, 5));
    println!("Answer of second question: {:?}", solve(input, 6));
}
fn solve(input: &str, zeroes: i32) -> u32 {
    let mut n: u32 = 0;
    let mut start_value = vec![];
    for _x in 0..zeroes / 2 {
        start_value.push(00);
    }
    if zeroes % 2 == 1 {
        start_value.push(15);
    }
    // println!("{:?}", start_value);
    loop {
        let string = input.to_owned() + &(n.to_string());

        let hash = md5::compute(&string);
        n += 1;
        // println!("{:?}", hash[0..start_value.len()].to_owned());
        if hash[0..start_value.len()].to_owned() <= start_value {
            return n;
        }
    }
}
