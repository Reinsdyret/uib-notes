use std::io;

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line);

    let numbers: Vec<&str> = line.trim().split(' ').collect();

    let a:u32 = numbers[0].parse().expect("Could not parse first num");
    let b:u32 = numbers[1].parse().expect("Could not parse second number");
    let c:u32 = numbers[2].parse().expect("Could not parse third number");

    for n in 1..c+1 {
        println!("{}", custom_fizzbuzz(a,b,n));
    }
}

fn custom_fizzbuzz(a: u32, b: u32, c:u32) -> String {
    let mut fizzbuzz = String::new();

    if c % a == 0 {
        fizzbuzz = "Fizz".to_owned();   
    }
    if c % b == 0 {
        fizzbuzz = fizzbuzz + "Buzz";
    }

    if fizzbuzz.len() == 0{
        return c.to_string();
    }

    fizzbuzz

}
