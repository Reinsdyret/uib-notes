
use std::{collections::{HashMap, HashSet}, io};
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
struct MemoKey {
    henchmen: HashSet<(usize, usize )>,
    need_to_bribe: usize,
    money: usize,
    cached_hash: u64,
}

impl MemoKey {
    fn new(henchmen: &HashSet<(usize, usize)>, need_to_bribe: usize, money: usize) -> Self {
        let mut memo_key = MemoKey {
            henchmen,
            need_to_bribe,
            money,
            cached_hash: 0,
        };
        memo_key.cache_hash();
        memo_key
    }

    fn cache_hash(&mut self) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.need_to_bribe.hash(&mut hasher);
        self.money.hash(&mut hasher);
        for h in self.henchmen {
            h.hash(&mut hasher);
        }
        self.cached_hash = hasher.finish();
    }
}

impl PartialEq for MemoKey {
    fn eq(&self, other: &Self) -> bool {
        self.need_to_bribe == other.need_to_bribe &&
        self.money == other.money &&
        self.henchmen == other.henchmen
    }
}

impl Hash for MemoKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.cached_hash);
    }
}


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
        let mut memo: HashMap<MemoKey, f32> = HashMap::new();
        println!("{}", percentage_complete(MemoKey::new(&henchmen, need_to_bribe, money), &mut memo));
    }
}

fn percentage_complete(memokey: MemoKey, memo: &mut HashMap<MemoKey, f32>) -> f32 {
    if memokey.need_to_bribe == 0 {return 1.0}

    if memokey.henchmen.len() == 0 || memokey.money == 0 {return 0.0}

    if (memo.contains_key(&memokey)) {
        return memo[&memokey]
    }

    let mut new_prob: f32 = 0.0;

    for (cost, probability) in memokey.henchmen.iter() {
        if cost <= &memokey.money {
            let probability1= (*probability as f32) * 0.01;
            let mut hench_clone = memokey.henchmen.clone();
            hench_clone.remove(&(cost.to_owned(), probability.to_owned()));
            let prob_success: f32 = percentage_complete(MemoKey::new(&hench_clone, memokey.need_to_bribe - 1, memokey.money - cost), memo) * probability1;
            let prob_fail: f32 = percentage_complete(MemoKey::new(&hench_clone, memokey.need_to_bribe, memokey.money - cost), memo) * (1.0 - probability1);

            new_prob = new_prob.max(prob_success + prob_fail);
        }
    }

    memo.insert(memokey, new_prob);

    return new_prob;
}
