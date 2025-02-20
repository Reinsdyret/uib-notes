
use std::{cmp, io};
use std::convert::TryInto;

fn main() {

    let [n, k]: [i32; 2]= get_n_i32(2).try_into().expect("Couldnt parse vec to arr");

    let mut intervals: Vec<(i32, i32)> = Vec::new();

    let mut max_time = 0;

    for _ in 0..n {
        let [a, b]: [i32; 2] = get_n_i32(2).try_into().expect("Couldnt parse vec to arr");
        if b > max_time {
            max_time = b;
        }
        intervals.push((a,b));
    }

    let mut max_intervals_in_window = 0;

    for window_start in 0..max_time + 1 {
        if k == 0 {
            // max_intervals_in_window = zero_window(&intervals); DOES NOT MAKE SENSE TO CARE ABOUT
            break;
        }
        let n = intervals_in_window(&intervals, (&window_start, cmp::min(&(window_start + k), &max_time)));
        //println!("Got {} intervals for window {} {}", n, window_start, window_start + k);
        if n > max_intervals_in_window {
            max_intervals_in_window = n;
        }
    }

    println!("{}", max_intervals_in_window);
}


fn zero_window(intervals: &Vec<(i32,i32)>) -> i32 {
    let mut counter:i32 = 0;

    for (start, stop) in intervals {
        if start <= &0 || stop <= &0 {
            counter += 1;
        }
    }

    return counter;
}

fn get_n_i32(n: i32) -> Vec<i32> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
}

fn intervals_in_window(intervals: &Vec<(i32,i32)>, window:(&i32,&i32)) -> i32 {

    let mut counter:i32 = 0;
    
    for (start, stop) in intervals {
        //println!("Start, stop: {} {} Window.0, window.1: {} {}",start, stop, window.0, window.1);
        if (start <= window.0 && stop >= window.0)|| (start <= window.1 && stop >= window.1) {
            //println!("Interval: {} {} Overlaps with window: {} {}", start, stop, window.0, window.1);
            counter += 1;
        }
    }

    return counter;
}