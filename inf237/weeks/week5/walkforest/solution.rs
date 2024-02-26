use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::TryInto;

// HEAVILY INSPIRED BY https://doc.rust-lang.org/std/collections/binary_heap/index.html
//  They also implement dijsktras so yes
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize
}

fn number_of_shortest_paths(adj_list: &Vec<Vec<Edge>>, start: usize, goal:usize) -> usize {
    // Combination of https://www.baeldung.com/cs/graph-number-of-shortest-paths
    // and https://doc.rust-lang.org/std/collections/binary_heap/index.html
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut paths: Vec<_> = (0..adj_list.len()).map(|_| 0).collect();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    dist[start] = 0;
    paths[start] = 1;
    heap.push(State { cost: 0, position: start });

    while let Some(State {cost, position}) = heap.pop() {
        if cost > dist[position] {continue;}

        for edge in &adj_list[position] {
            let next: State = State { cost: cost + edge.cost, position: edge.node};

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
                paths[next.position] = paths[position];
            }

            else if dist[next.position] == next.cost {
                paths[next.position] += paths[position];
            }
        }
    }

    //println!("{:?}", paths);
    return paths[goal];

}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    
    let mut line = String::new();
    let _ = stdin.read_line(&mut line);

    while line.trim() != "0" {
        let [n, m]: [usize; 2] = line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<usize>>().try_into().expect("Expected n and m");

        if n <= 1{
            println!("0");
            line = String::new();
            let _ = stdin.read_line(&mut line);
            continue;
        }

        let mut adj_list: Vec<Vec<Edge>> = Vec::new();

        for _ in 0..n + 1 {
            adj_list.push(Vec::new());
        }

        for _ in 0..m {
            let [a, b, d]: [usize; 3] = get_n_i32(3).try_into().expect("Expected 3 ints");
            adj_list[a].push(Edge { node: b, cost: d});
            adj_list[b].push(Edge {node: a, cost: d});
        }

        println!("{}", number_of_shortest_paths(&adj_list, 1, 2));

        line = String::new();
        let _ = stdin.read_line(&mut line);
    }
}

fn get_n_i32(n: i32) -> Vec<usize> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<usize>>();
}
