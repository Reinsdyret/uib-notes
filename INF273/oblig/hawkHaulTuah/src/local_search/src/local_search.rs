use std::{arch::x86_64, u32, usize};

use checker::checker::*;
use file_reader::parse_data::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub enum Operator {
    OneReinsert,
    TwoExchange,
    ThreeExchange
}

pub fn run_local_search(init_solution: &Vec<Vec<u32>>, operator: Operator, instance: &Instance) -> (Vec<Vec<u32>>, u128){
    let mut best_solution = init_solution.clone();
    let (mut best_cost, a) = check_feasibility_and_get_cost(&instance, &best_solution);
    
    for _i in 0 .. 10_000 {
        // let new_solution = one_reinsert_random(&best_solution);
        let new_solution = one_reinsert_focus_dummy_random(&best_solution, instance);

        let (cost, feasible) = check_feasibility_and_get_cost(instance, &new_solution);

        if feasible && cost < best_cost {
            best_solution = new_solution;
            best_cost = cost;
        }
    }

    return (best_solution, best_cost);
}

pub fn run_local_search_parallel(init_solution: &Vec<Vec<u32>>, operator: Operator, instance: &Instance) -> (Vec<Vec<u32>>, u128) {
    let best_solution = Arc::new(Mutex::new(init_solution.clone()));
    let best_cost = Arc::new(Mutex::new(check_feasibility_and_get_cost(instance, &init_solution).0));

    (0..10_000).into_par_iter().for_each(|_| {
        let current_solution = {
            let best_solution_lock = best_solution.lock().unwrap();
            best_solution_lock.clone()
        };

        let new_solution = one_reinsert_focus_dummy_random(&current_solution, instance);
        // let new_solution = one_reinsert_random(&current_solution);
        let (cost, feasible) = check_feasibility_and_get_cost(instance, &new_solution);

        if feasible {
            let mut best_cost_lock = best_cost.lock().unwrap();
            let mut best_solution_lock = best_solution.lock().unwrap();
            if cost < *best_cost_lock {
                *best_cost_lock = cost;
                *best_solution_lock = new_solution;
            }
        }
    });

    let best_solution = Arc::try_unwrap(best_solution).unwrap().into_inner().unwrap();
    let best_cost = Arc::try_unwrap(best_cost).unwrap().into_inner().unwrap();
    
    (best_solution, best_cost)
}

pub fn one_reinsert_random(old_solution: &Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    let mut rng = rand::thread_rng();
    let mut solution = old_solution.clone();

    let mut vehicle_from = rng.gen_range(0..solution.len());
    while solution[vehicle_from].is_empty() {
        vehicle_from = rng.gen_range(0..solution.len());
    }

    let call_idx = rng.gen_range(0..solution[vehicle_from].len());
    let call = solution[vehicle_from].remove(call_idx);
    
    if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
        solution[vehicle_from].remove(index);
    } else { panic!("There were not two calls in vehicle") }

    let vehicle_to = rng.gen_range(0..solution.len());
    let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
    solution[vehicle_to].insert(insert_idx, call);
    let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
    solution[vehicle_to].insert(insert_idx, call);
    
    return solution;
}


pub fn one_reinsert_focus_dummy_random(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let call: u32;
    let mut solution = old_solution.clone();
    let mut vehicle_from: usize = solution.len() - 1;
    // Choosing call, prioritizing outsource vehicle but still random call.
    // Random vehicle if outsource vehicle is empty
    if !solution[vehicle_from].is_empty() {
        let call_idx = rng.gen_range(0..solution[vehicle_from].len());
        call = solution[vehicle_from].remove(call_idx);
        
        // println!("Chose call {call}");
    } else {
        vehicle_from = rng.gen_range(0..solution.len());
        while solution[vehicle_from].is_empty() {
            vehicle_from = rng.gen_range(0..solution.len());
        }
        let call_idx = rng.gen_range(0..solution[vehicle_from].len());
        call = solution[vehicle_from].remove(call_idx);
    }

    if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
        solution[vehicle_from].remove(index);
    } else { panic!("There were not two calls in vehicle") }

    // Reinsert randomly
    let mut vehicle_to = rng.gen_range(0..solution.len());

    while !instance.compatibility[&((vehicle_to + 1) as u32)].contains(&call) {
        vehicle_to = rng.gen_range(0..solution.len());
    }
    // println!("Choose vehicle {vehicle_to}");

    // Insert into vehicle with least amount of calls
    let vehicle_to = get_index_least_calls(&solution);
    let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
    solution[vehicle_to].insert(insert_idx, call);
    let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
    solution[vehicle_to].insert(insert_idx, call);

    // Dont allow inserting on invalid vehicles

    return solution;
}

fn get_index_least_calls(solution: &Vec<Vec<u32>>) -> usize {
    let mut least_calls = usize::MAX;
    let mut least_index = usize::MAX;

    for (i, route) in solution[0 .. solution.len() - 1].iter().enumerate() {
        if route.len() < least_calls {
            least_calls = route.len();
            least_index = i;
        }
    }

    return least_index;
}


