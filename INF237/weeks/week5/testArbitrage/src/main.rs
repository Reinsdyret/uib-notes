
use std::{collections::HashMap, io};


fn main() {
    let mut line = get_line();

    while line.trim() != "0" {
        let n: i32 = line.trim().parse().expect("Not an int");
        let mut graph: Vec<(&str, &str, f32)> = Vec::new();
        
        line = get_line(); // Get vertices
        let vertices: Vec<&str> = line.split(' ').collect();
        let mut vertices2: Vec<&str> = Vec::new();
        for v in vertices {
            vertices2.push(&*v);
        }
        
        let cases: i32 = get_line().trim().parse().expect("Could not get int for cases");

        for _ in 0..cases {
            line = get_line(); // Reuse line for each case
            let u_v_w: Vec<&str> = line.split(' ').collect();
            let (u, v) = (u_v_w[0], u_v_w[1]);
            let weights: Vec<&str> = u_v_w[2].split(':').collect();
            let (u_w, v_w): (f32, f32) = (
                weights[0].parse().expect("Expected number for uw"),
                weights[1].parse().expect("Expected weight for v"),
            );

            let w = if u_w == v_w {
                0.0
            } else {
                -1.0 * f32::log2(v_w / u_w)
            };

            graph.push((u, v, w));
        }

        let mut is_arbitrage = false;

        for (v, _, _) in graph.iter(){
            if bellman_ford(&*vertices2.clone(), &v, &graph) {
                is_arbitrage = true;
            }
        }

        if is_arbitrage {
            println!("Arbitrage");
        } else {
            println!("Ok");
        }

        // Assuming the rest of your logic to check for arbitrage or other processing goes here
        // For now, just directly prepare for the next iteration
        line = get_line(); // Prepare for the next iteration at the end of the loop
    }
}

fn bellman_ford(vertices: &[&str], src: &str, g: &Vec<(&str, &str, f32)>) -> bool{
    let mut dist: HashMap<&str, f32> = HashMap::new();

    for v in vertices.iter() {
        dist.insert(&v, f32::INFINITY);
    }

    dist.insert(&src, 0.0);

    for _ in 0..vertices.len() - 1 {
        for (u, v, w) in g.iter() {
            if let Some(a) = dist.get(u){
                if let Some(b) = dist.get(v){
                    if a != &f32::INFINITY && a + w < *b {
                        dist.insert(v, a + w);
                    }
                }
            }
        }
    }

    for (u, v, w) in g.iter() {
        if let Some(a) = dist.get(u){
            if let Some(b) = dist.get(v){
                if a != &f32::INFINITY && a + w < *b {
                    return true;
                }
            }
        }
    }


    return false;
}

fn get_line() -> String {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().to_string();
}
