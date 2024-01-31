use std::io;
use std::cmp;


fn largest_diff_row(row1: &Vec<i32>, row2: &Vec<i32>) -> i32 {
    let mut largest: i32 = 0;

    for weight in row1 {
        if weight > &largest {
            if let Ok(x) = row2.binary_search(&weight) {
                largest = *weight;   
            }
        }
    }

    return largest;
}

fn largest_surrounded(row: &Vec<i32>) -> i32 {
    let mut largest: i32 = 0;

    for i in 1..row.len()-1 {
        if row[i] > largest && row[i] < row[i-1] && row[i] < row[i + 1] {
            let mut copy = row.clone();
            copy.remove(i);
            largest = cmp::max(row[i], largest_surrounded(&copy));
        }
    }

    return largest;
}

fn main() {
    let mut stdin = io::stdin();

    let mut line = String::new();

    let _ = stdin.read_line(&mut line);

    let n: i32 = line.trim().parse().expect("Expected i32");
    line = String::new();

    let _ = stdin.read_line(&mut line);

    let row1: Vec<i32> = line.trim().split(' ').map(|n| n.parse().expect("Expected i32")).collect();

    line = String::new();
    let _ = stdin.read_line(&mut line);

    let row2: Vec<i32> = line.trim().split(' ').map(|n| n.parse().expect("Expected i32")).collect();

    // Sorted copies
    let mut sorted_row1 = row1.clone();
    sorted_row1.sort_unstable();
    let mut sorted_row2 = row2.clone();
    sorted_row2.sort_unstable();

    let largest = cmp::max(largest_diff_row(&sorted_row1, &sorted_row2), cmp::max(largest_surrounded(&row1), largest_surrounded(&row2)));

    println!("{}", largest);
}
