use std::io;

fn main() {
    let stdin = io::stdin();

    let lines = stdin.lines();

    for line in lines{
        let unwrapped = line.unwrap();
        if unwrapped == ""{
            break;
        }
        let nums: Vec<&str> = unwrapped.trim().split(' ').collect();
        let a:i64 = nums[0].parse().expect("Couldnt parse first number");
        let b:i64 = nums[1].parse().expect("Couldnt parse second number");

        println!("{}",(a-b).abs());

    }
}
