use std::fs
use std:cmp
use std::collections::HashSet;
fn main() {
    let x: u32 = 0;
    let y: u32 = 0;
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
        let mut points = HashSet::new();
    for direction in contents.chars().into_iter() {
        match direction {
            '<' => x -= 1,
            '>' => y += 1,
            'v' => y -= 1,
            '^' => y += 1,
        }
        points.
        const highest_num:i32 = find_highest_num(nth_cube);
        const lowest_num:i32 = find_highest_num(nth_cube-1) -1
        
        hash.push()
    }
}
