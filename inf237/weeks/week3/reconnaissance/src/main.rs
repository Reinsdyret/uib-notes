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

    // Get one input first so we can compare largest and smallest
    let [x, v]: [f64; 2] = get_f64s().try_into().expect("Expected two ints");
    numbers.push((x, v));

    for i in 1..n as i64 {
        let [x, v]: [f64; 2] = get_f64s().try_into().expect("Expected two ints");
        numbers.push((x, v));
    }

    let x = gss(fp, 0.0, 1000.0, 0.0001, f64::sqrt(5.0) + 1.0 / 2.0, &numbers);

    println!("{}", fp(x, &numbers));
}

fn f(t: f64, values: &Vec<(f64,f64)>) -> f64{
    let mut largest: f64;
    let mut smallest: f64;

    let mut values_copy = values.clone();

    for i in 0..values.len() {
        values_copy[i].0 += values_copy[i].1 * t;
    }

    largest =  0.0;
    smallest = 0.0;
    

    for (pos, _) in values_copy {
        largest = if pos > largest {pos} else {largest};
        smallest = if pos < smallest {pos} else {smallest};
        println!("{} {}", largest, smallest);
    }

    println!("{} {}", largest, smallest);

    return diff(&largest, &smallest);
}

fn fp(t: f64, values: &Vec<(f64, f64)>) -> f64 {
    let mut min_distance = f64::INFINITY;

    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            let pos_i = values[i].0 + values[i].1 * t;
            let pos_j = values[j].0 + values[j].1 * t;

            let distance = (pos_i - pos_j).abs();
            min_distance = min_distance.min(distance);
        }
    }

    return min_distance;
}

fn get_f64s () -> Vec<f64> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<_>>();
}

fn diff(a: &f64, b: &f64) -> f64 {
    if (a >= &0.0 && b >= &0.0) || (a <= &0.0 && b <= &0.0) {
        return f64::max(a.abs(), b.abs()) - f64::min(a.abs(), b.abs());
    } else {
        return a.abs() + b.abs();
    }
}

fn gss(f: fn (f64, &Vec<(f64,f64)>) -> f64, mut a: f64, mut b: f64, tolerance: f64, gr: f64, values: &Vec<(f64, f64)>) -> f64{
    // Golden section search. Used pseudocode from https://en.wikipedia.org/wiki/Golden-section_search# 

    let mut c: f64;
    let mut d: f64;
    while f64::abs(b - a) > tolerance {
        c = b - (b - a) / gr;
        d = a + (b - a) / gr;

        if f(c, &values) < f(d, &values) {
            b = d;
        } else {
            a = c;
        }
        println!("a: {}, b: {}", a, b);
    }

    println!("({} + {}) / 2 = {}", a, b, (a + b) / 2.0);
    return ((b + a) / 2.0); 

}