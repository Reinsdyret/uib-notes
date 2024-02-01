
use std::{cmp, io};
use std::convert::TryInto;

fn main() {

    let [n, k]: [i32; 2]= get_n_i32(2).try_into().expect("Couldnt parse vec to arr");

    // A list on intervals where bool signifies whether the number is start or stop time
    // Start time is True and Stop False
    let mut intervals: Vec<(i32, bool)> = Vec::new();

    let mut max_time = 0;

    for _ in 0..n {
        let [a, b]: [i32; 2] = get_n_i32(2).try_into().expect("Couldnt parse vec to arr");
        if b > max_time {
            max_time = b;
        }
        intervals.push((a,true));
        // Increase stop time by k to just care about staying in the bar for 0 seconds.
        intervals.push(((b + k), false));
    }

    intervals.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

    let mut max_counter = 0;
    let mut counter = 0;

    for (time, start) in intervals {
        if start {
            counter += 1;
            max_counter = cmp::max(max_counter, counter);
        } else {
            counter -= 1;
        }
    }

    println!("{}", max_counter)
}

fn get_n_i32(n: i32) -> Vec<i32> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
}