use rand::prelude::*;
use std::collections::{ HashMap, HashSet, };

pub fn get_random_sol(num_calls: u32, num_vehicles: u32, compatibility: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut routes: Vec<Vec<u32>> = Vec::new();
    let mut seen: HashSet<u32> = HashSet::new();

    for _i in 0..num_vehicles {
        routes.push(Vec::new());
    }

    for call in 1 .. num_calls + 1 {
        let mut vehicle = rng.gen_range(1 .. num_vehicles + 1);
        
        
        while !compatibility[&vehicle].contains(&call) || seen.contains(&call) {
            vehicle = rng.gen_range(1 .. num_vehicles + 1);
        }

        routes[(vehicle - 1) as usize].push(call);
        seen.insert(call);
    }

    for route in &mut routes {
        let to_add: Vec<u32> = route.clone();
        route.extend(to_add);

        route.shuffle(&mut rng);
    }    

    // Put solutions together into one vec
    let mut solution = Vec::new();
    for route in routes[0 .. routes.len() - 1].into_iter() {
        solution.extend(route);
        solution.push(0);
    }

    solution.extend(routes[routes.len() - 1].clone());

    return solution;

}
