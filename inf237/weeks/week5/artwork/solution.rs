
use std::{collections::{HashMap, HashSet}, convert::TryInto, io};


struct UndirectedGraph {
    nodes_is_white: Vec<Vec<bool>>,
    n: i32,
    m: i32
}

impl UndirectedGraph {
    fn new(n: i32, m: i32) -> UndirectedGraph {
        let mut temp_vector: Vec<Vec<bool>> = Vec::new();

        for _ in 0..n {
            let mut row:Vec<bool> = Vec::new();
            for _ in 0..m {
                row.push(true);
            }
            temp_vector.push(row);
        }

        UndirectedGraph {
            nodes_is_white: temp_vector,
            n: n,
            m: m
        }
    }

    fn make_black(&mut self, v: (i32, i32)) {
        self.nodes_is_white[v.0 as usize][v.1 as usize] = false;
    }

    fn get_neighbours(&self, v: (i32, i32)) -> Vec<(i32, i32)>{
        let mut neighbours: Vec<(i32, i32)> = Vec::new();

        let possible_neighbours: Vec<(i32, i32)> = vec![(v.0 + 1, v.1), (v.0, v.1 + 1), (v.0 - 1, v.1), (v.0, v.1 - 1)];

        for (possible_x, possible_y) in possible_neighbours {
            if (possible_x >= 0 && possible_x < self.n) && (possible_y >= 0 && possible_y < self.m) && self.nodes_is_white[possible_x as usize][possible_y as usize] {
                neighbours.push((possible_x, possible_y))
            }
        }

        return neighbours;
    }

    fn dfs(&self, visited: &mut HashMap<(i32, i32), bool>, v: (i32, i32)) {
        visited.insert(v, true);

        for (x, y) in self.get_neighbours(v){
            if !visited.get(&(x,y)).unwrap() && self.nodes_is_white[x as usize][y as usize] {
                self.dfs(visited, (x,y))
            }
        }
    }

    fn connected_components(&self) -> i32 {
        let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

        for x in 0..self.nodes_is_white.len() {
            for y in 0..self.nodes_is_white[x].len() {
                visited.insert((x as i32,y as i32), false);
            }
        }

        let mut component_count: i32 = 0;

        for x in 0..self.nodes_is_white.len() {
            for y in 0..self.nodes_is_white[x].len() {
                if !visited[&(x as i32,y as i32)] && self.nodes_is_white[x][y] {
                    self.dfs(&mut visited, (x as i32,y as i32));
                    component_count += 1;
                }
            }
        }

        return component_count;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    
    let [n, m, q]: [i32; 3] = get_n_i32(3).try_into().expect("Couldnt parse vec to arr");

    let mut graph: UndirectedGraph = UndirectedGraph::new(n, m);
    for _ in 0..q {
        let [x1, y1, x2, y2]: [i32; 4] = get_n_i32(4).try_into().expect("Couldnt parse vec to arr");

        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                graph.make_black((x -1 ,y - 1))
            }
        }

        println!("{}", graph.connected_components());
    }

}

fn get_n_i32(n: i32) -> Vec<i32> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().split(' ').map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
}