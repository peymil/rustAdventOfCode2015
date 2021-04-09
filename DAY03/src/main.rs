use std::collections::HashSet;
use std::fs;
fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut robo_y: i32 = 0;
    let mut santa_x: i32 = 0;
    let mut robo_y: i32 = 0;
    let mut santa_y: i32 = 0;
    let mut robo_x: i32 = 0;
    let contents =
        fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file");

    //                                think coordinate system like cube
    // 24 9  10 11 12    24 > > > >  start point is origin like original coordinate system
    // 23 8  1  2  13     ^ ^ ^ > v   [0,1] is 1 [1,1] is 2 [-2,2] is 24
    // 22 7  0  3  14     ^ ^ 0 v v   every outer cube is n
    // 21 6  5  4  15     ^ < < < v   (3n)^2 - 1 is the highest number in the nth cube
    // 20 19 18 17 16     ^ < < < <   ()
    // [0]
    // [1,2,3,4,5,6,7,8]
    // [9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24]
    // +x = -5
    // +y = -7
    // -x = -1
    // -y = -3
    // 3,11,19
    let mut points = HashSet::new(); //first part
    let mut alone_santa_position = (0, 0); //first part
    let mut points2 = HashSet::new(); //second part
    let mut current_robot_position = (0, 0); //second part
    let mut current_santa_position = (0, 0); //second part
    let mut n = 0;
    points.insert((0, 0));
    points2.insert((0, 0));
    for direction in contents.chars().into_iter() {
        let mut x = 0;
        let mut y = 0;
        match direction {
            '<' => x = -1,
            '>' => x = 1,
            'v' => y = -1,
            '^' => y = 1,
            _ => panic!("error"),
        }
        alone_santa_position = (alone_santa_position.0 + x, alone_santa_position.1 + y);
        points.insert(alone_santa_position);
        if ((n % 2) == 1) {
            current_santa_position = (current_santa_position.0 + x, current_santa_position.1 + y);
            points2.insert(current_santa_position);
        } else {
            current_robot_position = (current_robot_position.0 + x, current_robot_position.1 + y);
            points2.insert(current_robot_position);
        }
        n += 1
    }
    println!("First answer: {}", points.len());
    println!("Second answer: {}", points2.len());
}
