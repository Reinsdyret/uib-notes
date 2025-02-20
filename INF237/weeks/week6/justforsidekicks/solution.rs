use std::{cmp::Reverse, collections::HashMap, convert::TryInto, hash::Hash, io, iter::Sum};

#[derive(Clone)]
struct SegmentTree {
    arr: Vec<i32>,
}

impl SegmentTree {
    fn new(arr: &Vec<i32>, n: usize) -> SegmentTree {
        let mut myarr = vec![0; n];
        myarr.extend(arr);

        SegmentTree { arr: myarr }
    }

    fn left(&self, x: usize) -> usize {
        x * 2
    }
    fn right(&self, x: usize) -> usize {
        x * 2 + 1
    }

    fn parent(&self, x: usize) -> usize {
        x / 2
    }

    fn index(&self, i: &usize) -> usize {
        self.arr.len() / 2 + i
    }

    fn fill(&mut self, op: fn(Vec<i32>) -> i32) {
        let internal = 1..self.arr.len() / 2;

        for idx in internal.rev() {
            self.arr[idx] = vec![self.arr[self.left(idx)], self.arr[self.right(idx)]].iter().sum();
        }
    }

    fn query(&self, l: usize, r: usize) -> i32 {
        let mut li: usize = self.index(&l);
        let mut ri: usize = self.index(&r);

        let mut total_nums: i32 = 0;
        total_nums += self.arr[li];
        if li != ri {
            total_nums += self.arr[ri];
        }

        let mut pl = self.parent(li);
        let mut pr = self.parent(ri);

        while pl != pr {
            pl = self.parent(li);
            pr = self.parent(ri);
            if li % 2 == 0 {
                total_nums += self.arr[pl];
            }
            if r % 2 == 1 {
                total_nums += self.arr[pr];
            }

            li = pl;
            ri = pr;
        }

        return total_nums;
    }

    fn set(&mut self, idx: usize, value: i32) {
        let mut i = self.index(&idx);

        self.arr[i] = value;

        i = self.parent(i);
        while i > 0 {
            self.arr[i] = vec![self.arr[self.left(i)], self.arr[self.right(i)]].iter().sum();
            i = self.parent(i);
        }
    }
}

fn main() {
    let [n, q]: [i32; 2] = get_n_i32(2).try_into().expect("Couldnt take input");
    let values = get_n_i32(6);
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    //println!("{}", line);
    //println!("{:?}", line.chars());
    let gems1: Vec<i32> = line.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    //println!("{:?}", gems1);

    let mut type_to_value: HashMap<i32, i32> = HashMap::new();
    for (i, v) in values.into_iter().enumerate() {
        type_to_value.insert(i as i32, v);
    }

    //println!("{:?}", type_to_value);

    let mut gems: Vec<i32> = Vec::new();
    for g in gems1 {
        gems.push(g - 1);
    }

    println!("{:?}", gems);

    let mut trees: Vec<SegmentTree> = Vec::new();
    for i in 0..6 {
        let mut tree: Vec<i32> = Vec::new();
        for i in 0..gems.len() {
            //println!("{}", gems[i]);
            if gems[i] == i as i32 {
                tree.push(1);
            } else {
                tree.push(0);
            }
        }

        //println!("{:?}", tree);

        trees.push(SegmentTree::new(&tree, tree.len()));
    }

    for _ in 0..q {
        let [op, v1, v2]: [i32; 3] = get_n_i32(3).try_into().expect("Couldnt parse vec as arr");

        if op == 1 {
            for (i, mut tree) in trees.clone().into_iter().enumerate(){
                if i as i32 == v2 - 1 {
                    //println!("Equal");
                    tree.set(v1 as usize - 1, 1);
                } else {
                    tree.set(v1 as usize - 1, 0);
                }

            }
        } else if op == 2 {
            type_to_value.insert(v1 - 1, v2);
        } else {
            let mut num = 0;
            for (i, mut tree) in trees.clone().into_iter().enumerate(){
                //println!("Query");
                num += tree.query(v1 as usize - 1, v2 as usize - 1) * type_to_value[&(i as i32)];
            }

            println!("{}", num);
        }
    }
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
