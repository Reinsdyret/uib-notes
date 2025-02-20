use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line);


    let nums: Vec<&str> = line.trim().split(' ').collect();
    let b:i32 = nums[0].parse().expect("Did not parse b");
    let k:i32 = nums[1].parse().expect("Did not parse k");
    let g:i32 = nums[2].parse().expect("Did not parse g");



    println!("{}",((b)/(k / g)));
}
