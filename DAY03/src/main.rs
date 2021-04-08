use std::collections::HashMap;

fn main() {
    let mut uniquePoints = HashMap::new();
    let mut sum = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let contents =
        std::fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file");
    uniquePoints.insert((0, 0), true);
    sum += 1;
    for direction in contents.chars().into_iter() {
        match direction {
            '<' => x -= 1,
            '>' => y += 1,
            'v' => y -= 1,
            '^' => y += 1,
            _ => panic!("error"),
        }
        uniquePoints.entry((x, y)).or_insert_with(|| {
            sum += 1;
            return true;
        });
    }
    println!("{}", sum)
}
