use std::io;
use std::str::FromStr;
use std::cmp;
use std::fmt::Debug;
use std::convert::TryInto;

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();
    let _ = stdin.read_line(&mut n);

    for _ in 0..n.trim().parse::<u32>().expect("fail parse"){
        let mut nums: Vec<i64> = vec![];
        collect_ints::<i64>(2, &mut nums);
        let length = nums[0];
        let ants = nums[1];

        let mut ants_position: Vec<i64> = vec![];
        collect_ints::<i64>(ants, &mut ants_position);
        let shortest_time: i64 = ants_position
            .iter()
            .map(|&pos| cmp::min(pos, length - pos))
            .max()
            .unwrap();

        let max_pos = *ants_position.iter().max().unwrap();
        let min_pos = *ants_position.iter().min().unwrap();

        let longest_time: i64 = length - cmp::min(length - max_pos, min_pos);

        println!("{} {}", shortest_time, longest_time);

    }
}

fn collect_ints<T: FromStr>(n: i64, current: &mut Vec<T>) -> Result<(), T::Err> where <T as FromStr>::Err: Debug{
    // Inspired by https://users.rust-lang.org/t/collecting-a-vector-from-stdin-and-parsing-them-as-f64-or-f32-or-i64-etc/16252/2 
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).expect("Could not read from stdin");
    let mut parsed: Vec<T> = line.trim().split_whitespace().map(|word| word.parse().unwrap()).collect::<Vec<_>>();

    current.append(&mut parsed);

    if current.len() >= n.try_into().unwrap() {
        return Ok(());
    } else {
        return collect_ints(n, current);
    }
    
}
