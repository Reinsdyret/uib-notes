use std::io;

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line);

    let time: Vec<i32> = line.trim().split(' ').map(|num| num.parse().unwrap()).collect();

    if time[1] > 45{
        println!("{} {}", time[0], time[1] - 45);
    } else {
        println!("{} {}", (24 + time[0] - 1) % 24, (60 + time[1] - 45) % 60);
    }
}
