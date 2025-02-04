use rand::prelude::*;
use std::collections::{ HashMap, HashSet, };

pub fn get_random_sol(num_calls: u32, num_vehicles: u32, valid_vehicles: &HashMap<u32, Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut routes: Vec<Vec<u32>> = vec![Vec::new(); num_vehicles as usize];
    let mut seen: HashSet<u32> = HashSet::new();


    for call in 1 .. num_calls + 1 {
        if let Some(vehicles) = valid_vehicles.get(&call) {
            let vehicle = vehicles[rng.gen_range(0 .. vehicles.len())];
            routes[(vehicle -1) as usize].push(call);
            seen.insert(call);
        }
    }

    for route in &mut routes {
        route.extend_from_within(.. route.len());

        route.shuffle(&mut rng);
    }

    return routes;

}
