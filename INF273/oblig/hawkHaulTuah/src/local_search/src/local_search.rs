use std::{arch::x86_64, u32, usize};

use checker::checker::*;
use file_reader::parse_data::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::operators::*;

pub enum Operator {
    OneReinsert,
    TwoExchange,
    ThreeExchange,
}

pub fn run_local_search(
    init_solution: &Vec<Vec<u32>>,
    operator: Operator,
    instance: &Instance,
) -> (Vec<Vec<u32>>, u128) {
    let mut best_solution = init_solution.clone();
    let (mut best_cost, a) = check_feasibility_and_get_cost(&instance, &best_solution);

    for _i in 0..10_000 {
        // let new_solution = one_reinsert_random(&best_solution);
        let new_solution = one_reinsert_focus_dummy_random_feasible(&best_solution, instance);

        let (cost, feasible) = check_feasibility_and_get_cost(instance, &new_solution);

        if feasible && cost < best_cost {
            best_solution = new_solution;
            best_cost = cost;
        }
    }

    return (best_solution, best_cost);
}

pub fn run_local_search_parallel(
    init_solution: &Vec<Vec<u32>>,
    operator: Operator,
    instance: &Instance,
) -> (Vec<Vec<u32>>, u128) {
    let best_solution = Arc::new(Mutex::new(init_solution.clone()));
    let best_cost = Arc::new(Mutex::new(
        check_feasibility_and_get_cost(instance, &init_solution).0,
    ));

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

    let best_solution = Arc::try_unwrap(best_solution)
        .unwrap()
        .into_inner()
        .unwrap();
    let best_cost = Arc::try_unwrap(best_cost).unwrap().into_inner().unwrap();

    (best_solution, best_cost)
}
