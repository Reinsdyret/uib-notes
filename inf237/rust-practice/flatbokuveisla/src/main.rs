use std::io;

fn main() {
    let mut slices = String::new();
    let mut people = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut slices).expect("No");
    
    stdin.read_line(&mut people).expect("No");

    let slicesI: i32 = slices.trim().parse().expect("Not a valid number");
    let peopleI: i32 = people.trim().parse().expect("Not a valid number");

    let result = slicesI % peopleI;

    println!("{}", result);
}
