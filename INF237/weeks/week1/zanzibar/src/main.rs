use std::io;

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();
    stdin.read_line(&mut n);

    for _ in 0..n.trim().parse::<u32>().expect("NO"){
        let mut line = String::new();
        stdin.read_line(&mut line);
        let nums:Vec<i64> = line.trim().split(' ').map(|num| num.parse().unwrap()).collect();
        let mut imported:i64 = 0;
        for i in 0..nums.len()-2 {
            if nums[i+1] - (nums[i] * 2) < 0 {
                continue;
            }
            imported += nums[i+1] - (nums[i] * 2);
        }

        println!("{}", imported);
    }
}
