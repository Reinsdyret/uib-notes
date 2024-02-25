
use std::io;
mod scanner;

use scanner::Scanner;

struct UndirectedGraph {
    nodes_is_white: Vec<Vec<bool>>
}

impl UndirectedGraph {
    
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let x = scan.token::<i32>();
    println!("{}", x);
}
