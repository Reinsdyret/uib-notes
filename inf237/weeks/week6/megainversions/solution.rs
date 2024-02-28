
use std::{convert::TryInto, io};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());

    let [n]: [i32; 1] = get_n_i32(1).try_into().expect("Didnt recieve n");

    let arr: Vec<i32> = get_n_i32(n);

    println!("{}", count_triple_inversions(n, arr));
}

fn count_triple_inversions(n: i32, arr: Vec<i32>) -> usize{
    let mut count: usize = 0;

    for i in 1..n-1{
        let mut smaller: usize = 0;

        for j in i+1..n {
            if arr[j as usize] < arr[i as usize] {
                smaller += 1;
            }
        }

        let mut greater: usize = 0;

        for j in (0..i).rev() {
            if arr[j as usize] > arr[i as usize] {
                greater += 1;
            }
        }

        count += smaller * greater;
    }

    return count;
}


fn get_n_i32(n: i32) -> Vec<i32> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line
        .trim()
        .split(' ')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<i32>>();
}