use std::{collections::HashMap, io};


fn main() {
    let mut line = get_line();

    while line.trim() != "0" {
        let n: i32 = line.trim().parse().expect("Not an int");
        let mut graph: Vec<(String, String, f32)> = Vec::new();
        
        line = get_line(); // Get vertices

        let mut vertices: Vec<String> = line.split(' ').map(|a| a.to_owned()).collect();
        
        let cases: i32 = get_line().trim().parse().expect("Could not get int for cases");

        for _ in 0..cases {
            line = get_line(); // Reuse line for each case

            let u_v_w: Vec<&str> = line.split(' ').collect();
            let (u, v) = (u_v_w[0].to_owned(), u_v_w[1].to_owned());
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

        if cases > 1 && bellman_ford(&vertices, &graph[0].0, &graph) {
            is_arbitrage = true;
        }

        if is_arbitrage {
            println!("Arbitrage");
        } else {
            println!("Ok");
        }

        line = get_line(); // Prepare for the next iteration at the end of the loop

    }
}

fn bellman_ford(vertices: &Vec<String>, src: &str, g: &Vec<(String, String, f32)>) -> bool{
    let mut dist: HashMap<String, f32> = HashMap::new();

    for v in vertices {
        dist.insert(v.to_string(), f32::INFINITY);
    }

    dist.insert(src.to_string(), 0.0);

    for _ in 0..&vertices.len() - 1 {
        for (u, v, w) in g.iter() {
            if let Some(a) = dist.get(u){
                if let Some(b) = dist.get(v){
                    if a != &f32::INFINITY && a + w < *b {
                        dist.insert(v.to_string(), a + w);
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