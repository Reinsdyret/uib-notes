use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Expected int");

    let mut shots: Vec<(f64, f64)> = Vec::new();

    for line in lines {
        let items: Vec<f64> = line.unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        shots.push((items[0], items[1]));
    }

    let mut max_dist = 0.0;

    for i in 0..n {
        for j in i+1..n {
            let (x1, y1) = shots[i];
            let (x2, y2) = shots[j];

            let dist = f64::powi(x2 - x1, 2) + f64::powi(y2 - y1, 2);

            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    println!("{}", f64::sqrt(max_dist));
}
