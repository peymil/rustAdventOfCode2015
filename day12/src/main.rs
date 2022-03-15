use std::fs;
fn main() {
    let content = fs::read_to_string("./input.json").expect("Something went wrong reading the file");
    let mut first_result:i32 = 0;
    for line in content.lines(){
        let mut str_num = String::new();
        for chr in line.chars(){
            if chr.is_numeric() || chr == '-' {
                str_num.push(chr);
            } else if !str_num.is_empty() {
                first_result += str_num.parse::<i32>().unwrap();
                str_num.clear();
            }
            println!("{}",str_num.is_empty());
        }
    }

    println!("{}",first_result);
}
