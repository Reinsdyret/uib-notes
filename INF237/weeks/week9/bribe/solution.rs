use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let cases: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..cases {
        let line = lines.next().unwrap().unwrap();
        let items: Vec<usize> = line.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
        let (number_henchmen, need_to_bribe, money) = (items[0], items[1], items[2]);

        let mut henchmen: Vec<(usize, usize)> = Vec::new();
        for _ in 0..number_henchmen {
            let line = lines.next().unwrap().unwrap();
            let i: Vec<usize> = line.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
            let (cost, percentage) = (i[0], i[1]);
            henchmen.push((cost, percentage));
        }

        let mut memo: HashMap<(u16, usize, usize), f64> = HashMap::new();
        let result = percentage_complete(&henchmen, 0, need_to_bribe, money, &mut memo);
        println!("{}", result);
    }
}

fn percentage_complete(henchmen: &[(usize, usize)], used_mask: u16, need_to_bribe: usize, money: usize, memo: &mut HashMap<(u16, usize, usize), f64>) -> f64 {
    if need_to_bribe == 0 {
        return 1.0;
    }
    if money == 0 {
        return 0.0;
    }

    let key = (used_mask, need_to_bribe, money);
    if let Some(&prob) = memo.get(&key) {
        return prob;
    }
    
    //if used_mask.count_ones() as usize > need_to_bribe {
    //    memo.insert(key, 0.0);
    //    return 0.0;
    //}

    let mut new_prob: f64= 0.0;
    for (i, (cost, percentage)) in henchmen.iter().enumerate() {
        let mask = 1 << i;
        if used_mask & mask == 0 && *cost <= money {
            let probability :f64= (*percentage as f64) / 100.0;
            let prob_success = percentage_complete(henchmen, used_mask | mask, need_to_bribe - 1, money - cost, memo) * probability;
            let prob_fail = percentage_complete(henchmen, used_mask | mask, need_to_bribe, money - cost, memo) * (1.0 - probability);

            new_prob = new_prob.max(prob_success + prob_fail);
        }
    }

    memo.insert(key, new_prob);
    new_prob
}
