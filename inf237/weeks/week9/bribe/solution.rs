
use std::{collections::{HashMap, HashSet}, io};


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    
    let mut line = String::new();
    stdin.read_line(&mut line);

    let cases: usize = line.trim().parse().unwrap();

    for case in 0..cases {
        line.clear();

        stdin.read_line(&mut line);
        let items: Vec<usize> = line.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
        let (number_henchmen, need_to_bribe, money) = (items[0], items[1], items[2]);

        let mut henchmen: HashSet<(usize, usize)> = HashSet::new();
        for _ in 0..number_henchmen {
            line.clear();
            stdin.read_line(&mut line);

            let i: Vec<usize> = line.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
            let (cost, percentage) = (i[0], i[1]);
            henchmen.insert((cost, percentage));
        }
        let mut memo: HashMap<(String, usize, usize), f32> = HashMap::new();
        println!("{}", percentage_complete(&mut henchmen, need_to_bribe, money, &mut memo));
    }
}

fn percentage_complete(henchmen: &mut HashSet<(usize, usize)>, need_to_bribe: usize, money: usize, memo: &mut HashMap<(&mut HashSet<(usize, usize)>, usize, usize), f32>) -> f32 {
    if need_to_bribe == 0 {return 1.0}

    if henchmen.len() == 0 || money == 0 {return 0.0}

    if (memo.contains_key(&(henchmen, need_to_bribe, money))) {
        return memo[&(henchmen, need_to_bribe, money)]
    }

    let mut new_prob: f32 = 0.0;

    for (cost, probability) in henchmen.iter() {
        if cost <= &money {
            let mut hench_clone = henchmen.clone();
            hench_clone.remove(&(cost.to_owned(), probability.to_owned()));
            let prob_success: f32 = percentage_complete(&mut hench_clone, need_to_bribe - 1, money - cost, memo) * (*probability as f32);
            let prob_fail: f32 = percentage_complete(&mut hench_clone, need_to_bribe, money - cost, memo) * (1.0 - *probability as f32);

            new_prob = new_prob.max(prob_success + prob_fail);
        }
    }

    memo.insert((stringify!(henchmen).to_owned(), need_to_bribe, money), new_prob);

    return new_prob;
}
