
use std::{collections::HashMap, io};


fn main() {
    
    let mut line = get_line();

    let mut graph: Vec<(&str, &str, f32)>;
    let mut vertices: Vec<&str>;
    let mut n: i32;
    let mut cases: i32;

    while line.trim() != "0" {
        graph = Vec::new();
        n = line.trim().parse().expect("Not an int");
        line = get_line();
        vertices = line.split(' ').collect();
        cases = get_line().parse().expect("Could not get itn for cases");

        for _ in 0..cases {
            line = get_line();
            let u_v_w:Vec<&str> = line.split(' ').collect();
            let u: &str = u_v_w[0].to_owned();
            let v: &str = u_v_w[1].to_owned();
            let weights: Vec<&str> = u_v_w[2].split(':').collect();
            let u_w: f32 = weights[0].parse().expect("Expected number for uw");
            let v_w: f32 = weights[1].parse().expect("Expected weight for v");

            let w: f32;
            if u_w == v_w {
                w = 0.0;
            } else {
                w = -1.0 * f32::log(v_w / u_w, 2.0);
            }

            graph.push((u, v, w));
        }

        let mut is_arbitrage = false;

        for (v, _, _) in graph.iter(){
            if bellman_ford(&vertices, &v, &graph) {
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

fn bellman_ford(vertices: &Vec<&str>, src: &str, g: &Vec<(&str, &str, f32)>) -> bool{
    let mut dist: HashMap<&str, f32> = HashMap::new();

    for v in vertices {
        dist.insert(&v, f32::INFINITY);
    }

    dist.insert(&src, 0.0);

    for _ in 0..&vertices.len() - 1 {
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
