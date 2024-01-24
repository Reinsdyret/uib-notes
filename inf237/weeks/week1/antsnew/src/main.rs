use std::io;
use std::cmp;

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();
    let _ = stdin.read_line(&mut n);
    let mut line = String::new();

    for _ in 0..n.trim().parse::<u32>().unwrap() {
        line = "".to_string();
        let _ = stdin.read_line(&mut line);
        let nums: Vec<&str> = line.trim().split(' ').collect();

        let length: u32 = nums[0].parse().unwrap();
        let ants: usize = nums[1].parse().unwrap();

        let mut ant_position: Vec<u32> = vec![];

        while ant_position.len() < ants{
            line = "".to_string();
            let _ = stdin.read_line(&mut line);
            let mut new_pos: Vec<u32> = line.trim()
                .split(' ')
                .map(|pos| pos.parse().unwrap())
                .collect();

            ant_position.append(&mut new_pos);
        }

        let mut min_pos: u32 = ant_position[0];
        let mut max_pos: u32 = ant_position[0];
        let mut shortest_time: u32 = cmp::min(ant_position[0], length - ant_position[0]);
        let mut buff: u32 = 0;

        for &pos in ant_position.iter() {
            buff = cmp::min(pos, length - pos);
            if buff > shortest_time {
                shortest_time = buff;
            }

            if pos < min_pos {
                min_pos = pos;
            }

            if pos > max_pos {
                max_pos = pos;
            }
        }

        let longest_time: u32 = length - cmp::min(length - max_pos, min_pos);

        println!("{} {}", shortest_time, longest_time);
    }
}
