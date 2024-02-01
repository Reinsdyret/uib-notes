use std::{cmp, io};
use std::convert::TryInto;
use std::process;

fn main() {

    let [n]: [i32;1] = get_i32s().try_into().expect("Expected 1 int");

    let mut largest: usize = 0;
    let mut smallest: usize = 0;
    let mut numbers: Vec<(i32, i32)> = Vec::new();

    let mut all0v: bool = true;
    // Get one input first so we can compare largest and smallest
    let [x, v]: [i32; 2] = get_i32s().try_into().expect("Expected two ints");
    numbers.push((x, v));

    for i in 1..n {
        let [x, v]: [i32; 2] = get_i32s().try_into().expect("Expected two ints");
        numbers.push((x, v));
        if x > numbers[largest].0 {
            largest = i as usize;
        } else if x < numbers[smallest].0 {
            smallest = i as usize;
        }

        if v != 0{
            all0v = false
        }
    }

    numbers.sort();

    let mut prev_distance = diff(&numbers[largest].0, &numbers[smallest].0);

    if all0v{
        println!("{}", prev_distance);
        process::exit(0);
    }

    while diff(&numbers[largest].0, &numbers[smallest].0) <= prev_distance {
        for i in 0..numbers.len() {
            numbers[i].0 += numbers[i].1;
            if numbers[i].0 > numbers[largest].0 {
                largest = i;
            } else if numbers[i].0 < numbers[smallest].0 {
                smallest = i;
            }
        }

        //println!("{:?}, smallest:{}, largest:{}, diff: {}", numbers, numbers[smallest].0, numbers[largest].0, diff(&numbers[largest].0, &numbers[smallest].0));

        if diff(&numbers[largest].0, &numbers[smallest].0) > prev_distance {
            break;
        }


        prev_distance = diff(&numbers[largest].0, &numbers[smallest].0);
    }

    println!("{}", prev_distance);
}


fn get_i32s() -> Vec<i32> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
}

fn diff(a: &i32, b: &i32) -> i32 {
    if (a >= &0 && b >= &0) || (a <= &0 && b <= &0) {
        return cmp::max(a.abs(), b.abs()) - cmp::min(a.abs(), b.abs());
    } else {
        return a.abs() + b.abs();
    }
}