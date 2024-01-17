use std::io;

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();

    stdin.read_line(&mut n).expect("Couldnt read line");

    let parsed_n:usize = n.trim().parse().expect("Couldnt parse n");

    let result = "0.".to_owned() + &"0".repeat(parsed_n-1) + &"1";

    println!("{}", result);
}
