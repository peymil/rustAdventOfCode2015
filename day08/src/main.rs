use std::fs;


fn main() {
    let content = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let mut literal_count = 0;
    let mut char_count = 0;
    let mut encoded_count = 0;
    for line in content.lines(){
        let mut chars_iterator = line.chars().peekable();
        let mut is_in_escape = false;
        // Remove quotes
        // If line is empty, continue
        if chars_iterator.next() == None || chars_iterator.next_back() == None {
            continue;
        };
        
        literal_count += 2;
        encoded_count += 4;
        while let Some(chr) = chars_iterator.next() { 
            literal_count += 1;
            if is_in_escape {
                if chr == 'x' {
                        let mut hex_num = String::from("");
                        hex_num.push(chars_iterator.next().unwrap());
                        hex_num.push(chars_iterator.next().unwrap());
                        literal_count += 2;
                        if is_hex(hex_num){
                            char_count += 1;
                        }
                        else {
                            char_count += 4;
                        }
            
                }
                else if chr == '"' { char_count += 1; encoded_count += 1; }
                else if chr == '\\' { char_count += 1; encoded_count += 1; }
                else { char_count += 2; }
                is_in_escape = false;
            }
             else if chr == '\\' { is_in_escape = true; encoded_count += 1; }
             else { char_count += 1; }
        }
        println!("{}",line);
    }
    println!("{}", literal_count - char_count);
    println!("{}", encoded_count);
}
fn is_hex(hex_str: String) -> bool{
    let mut is_hex = true;
    for chr in hex_str.chars(){
        let chr_as_dec = chr as u8;
        if !((chr_as_dec >= b'0' && chr_as_dec <= b'9') || (chr_as_dec >= b'a' && chr_as_dec <= b'z')) {
            is_hex = false;
        }
    }
    is_hex
}