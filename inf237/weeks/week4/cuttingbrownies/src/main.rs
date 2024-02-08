use std::{io::{self, stdin, Read}, path::Iter};

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    let _ = stdin.read_line(&mut line);

    let n: i32 = line.trim().parse().expect("Expected int on first line");

    let mut brownies: Vec<(i32, i32, &str)> = Vec::new();

    for _ in 0..n {
        //line = String::new();
        //let _ = stdin.read_line(&mut line);
        line = get_line();
        let items: Vec<&str> = line.split(' ').collect();
        let width: i32 = items[0].parse().expect("Expected int");
        let height: i32 = items[1].parse().expect("Expected int");
        let person: &str = items[2];
        brownies.push((width, height, &person));
    }
}

fn get_line() -> String {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().to_string();
}

fn C(brownie: &(i32, i32, &str)) -> i32 {
    if brownie.2 == "Harry" {
        brownie.1
    } else {
        brownie.0
    }
}