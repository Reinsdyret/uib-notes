use checker::checker::*;
use file_reader::parse_data::Instance;
use rand::prelude::*;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    *,
};
use std::collections::{HashMap, HashSet};

fn get_random_sol(
    num_calls: u32,
    num_vehicles: u32,
    valid_vehicles: &HashMap<u32, Vec<u32>>,
) -> Vec<Vec<u32>> {
    let mut rng = rand::rng();
    let mut routes: Vec<Vec<u32>> = vec![Vec::new(); num_vehicles as usize];
    let mut seen: HashSet<u32> = HashSet::new();

    for call in 1..num_calls + 1 {
        if let Some(vehicles) = valid_vehicles.get(&call) {
            let vehicle = vehicles[rng.random_range(0..vehicles.len())];
            routes[(vehicle - 1) as usize].push(call);
            seen.insert(call);
        }
    }

    for route in &mut routes {
        route.extend_from_within(..route.len());

        route.shuffle(&mut rng);
    }

    return routes;
}

pub fn run_random(instance: &Instance) -> (u128, Vec<Vec<u32>>) {
    let results: Vec<(u128, bool, Vec<Vec<u32>>)> = (0..10_000)
        .into_par_iter()
        .map(|_| {
            let solution = get_random_sol(
                instance.num_calls,
                instance.num_vehicles,
                &instance.valid_vehicles,
            );
            let (cost, feasible) = check_feasibility_and_get_cost(&instance, &solution);
            return (cost, feasible, solution);
        })
        .collect();

    let (best_cost, _, best_solution) = results
        .into_iter()
        .filter(|(_, f, _)| *f)
        .min_by_key(|(cost, _, _)| *cost)
        .unwrap_or_else(|| (u128::MAX, false, vec![]));

    return (best_cost, best_solution);
}
