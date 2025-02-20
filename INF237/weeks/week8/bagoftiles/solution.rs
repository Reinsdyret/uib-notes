

use std::{f32::NAN, io};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());

    let mut line: String = String::new();
    stdin.read_line(&mut line);

    let games: usize = line.trim().parse().unwrap();

    for game in 1..games+1 {
        line.clear();

        stdin.read_line(&mut line);
        let n_tiles: usize = line.trim().parse().unwrap();

        line.clear();
        stdin.read_line(&mut line);
        let tiles: Vec<usize> = line.trim().split_whitespace().map(|t| t.parse().unwrap()).collect();
        
        line.clear();
        stdin.read_line(&mut line);
        let n_and_t: Vec<usize> = line.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
        let (n, t) = (n_and_t[0], n_and_t[1]);


        // Now dp
        let mut dp: Vec<Vec<usize>> = vec![vec![0; t + 1]; n + 1];
        dp[0][0] = 1;
        
        for tile in tiles {
            for n_tile in (1..n+1).rev() {
                for target in (0..t+1).rev() {
                    if target >= tile {
                        dp[n_tile][target] += dp[n_tile - 1][(target - tile)]
                    }
                }
            }
        }

        let number_wins = dp[n][t];
        let number_lose = binomial_coefficient(n_tiles, n) - number_wins;

        println!("Game {} -- {} : {}", game, number_wins, number_lose);

    }
}


fn binomial_coefficient(n: usize, k: usize) -> usize {
    let mut result = 1; 
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}