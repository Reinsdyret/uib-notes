mod operators;

fn main() {
    println!("Hello, world!");
    let mut solution:Vec<Vec<u32>> = vec![vec![4, 4, 2, 2], vec![5, 5, 7, 7], vec![1, 3, 1, 3], vec![6, 6]];
    operators::one_reinsert(&mut solution);

    println!("{solution:?}");
}
