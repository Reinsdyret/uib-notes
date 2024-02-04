use core::num;
use std::{cmp, io};
use std::convert::TryInto;
use std::process;

fn main() {

    let [n]: [f64;1] = get_f64s().try_into().expect("Expected 1 int");

    if n == 1.0 {
        println!("0");
        process::exit(0);
    }

    let mut numbers: Vec<(f64, f64)> = Vec::new();

    for _ in 0..n as i64 {
        let [x, v]: [f64; 2] = get_f64s().try_into().expect("Expected two ints");
        numbers.push((x, v));
    }

    // TODO: I cant increase the right limit more than 1000 before the first test stops :(
    let t = ternary_search(diff, 0.0, 1000.0, 0.0001, &numbers.clone());

    println!("{}", diff(t, &numbers));
}

fn get_f64s () -> Vec<f64> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<_>>();
}

fn diff(t: f64, values: &Vec<(f64,f64)>) -> f64 {
    let mut right: f64 = 0.0;
    let mut left: f64 = 0.0;

    for (pos, v) in values {
        let new_pos = pos + v * t;
        if new_pos >= right {
            right = new_pos;
        } else if new_pos <= left {
            left = new_pos;
        }
    }

    return right - left;
}

fn ternary_search(f: fn (f64, &Vec<(f64,f64)>) -> f64, mut left: f64, mut right: f64, tolerance: f64, values: &Vec<(f64, f64)>) -> f64{
    // Ternary search. Used pseudocode from https://en.wikipedia.org/wiki/Ternary_search

    let mut left_third: f64;
    let mut right_third: f64;

    while f64::abs(right - left) > tolerance {
        
        left_third = left + (right - left) / 3.0;
        right_third = right - (right - left) / 3.0;

        if f(left_third, &values) < f(right_third, &values) {
            right = left_third;
        } else {
            left = right_third;
        }
    }

    return right; 
}