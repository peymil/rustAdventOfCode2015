use regex::Regex;
use std::fs;


#[allow(arithmetic_overflow)]
fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
        //regex is uneffiecent
    let re = Regex::new(r"([\w\s]*?) (\d*),(\d*) through (\d*),(\d*)").unwrap();
    let mut santaLambs = Grid::new(1000,1000);
    let mut santaLambsBrighthness = Grid::new(1000,1000);
    for line in contents.lines(){
        let captured = re.captures(line).unwrap();
        let com = captured.get(1).unwrap().as_str();
        let x1 = captured.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let y1 = captured.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let x2 = captured.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let y2 = captured.get(5).unwrap().as_str().parse::<u32>().unwrap();
        match com {
            "turn on" => {
                santaLambs.set_values_of_square(x1, y1, x2, y2, 1);
                santaLambsBrighthness.add_to_values_in_square(x1, y1, x2, y2, 1);
            }
            "turn off" => {
                santaLambs.set_values_of_square(x1, y1, x2, y2, 0);
                santaLambsBrighthness.add_to_values_in_square(x1, y1, x2, y2, -1);
            }
            "toggle" => {
                santaLambs.toggle_values_of_square(x1, y1, x2, y2);
                santaLambsBrighthness.add_to_values_in_square(x1, y1, x2, y2, 2);
            }
            _ => {panic!("Undefined Command")}
        }
    }
    println!("{:?}",santaLambs.get_sum());
    println!("{:?}",santaLambsBrighthness.get_sum());
}

struct Grid(Vec<Vec<u8>>);
impl Grid {
    fn new(xlen:u32,ylen:u32) -> Self {
        Self(vec!(vec!(0;xlen as usize);ylen as usize))
    }

    fn set_values_of_square(&mut self,x1:u32,y1:u32,x2:u32,y2:u32,val:u8){
        for y in y1..y2+1{
            for x in x1..x2+1 {
                self.0[y as usize][x as usize] = val;
            }
        }
    }

    fn add_to_values_in_square(&mut self,x1:u32,y1:u32,x2:u32,y2:u32,val:i8){
        for y in y1..y2+1{
            for x in x1..x2+1 {
                let old_value = self.0[y as usize][x as usize] as i8; 
                let new_value = old_value + val;
                if(new_value >= 0){
                    self.0[y as usize][x as usize] = (old_value + val) as u8;
                }
                }
            
        }
    }
    fn toggle_values_of_square(&mut self,x1:u32,y1:u32,x2:u32,y2:u32){
        for y in y1..y2+1{
            for x in x1..x2+1 {
                let old_value = self.0[y as usize][x as usize];
                self.0[y as usize][x as usize] = old_value ^ 1;
            }
        }
    }
    fn get_sum(self) -> u32{
        let mut count:u32 = 0;
        for arr in self.0{
            for val in arr{
                count += val as u32;
            }
        }
        count
    }
}