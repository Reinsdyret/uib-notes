use std::f64;

use checker::checker::*;
use file_reader::parse_data::Instance;
use local_search::operators::*;
use rand::{random, thread_rng, Rng};

pub fn run_sa(
    init_solution: &Vec<Vec<u32>>,
    instance: &Instance,
    prob: f64,
    t_final: f64,
) -> (Vec<Vec<u32>>, u128) {
    let (delta_avg, mut incumbent, mut best_solution) =
        find_avg_delta(init_solution, instance, prob);

    let t_zero = (-1.0 * delta_avg) / (0.8f64).ln();

    let alpha = f64::powf(t_final / t_zero, 1.0 / 9900.0);
    let mut temp = t_zero;

    let mut new_solution: Vec<Vec<u32>>;
    let mut delta_e: f64;
    let mut best_cost = check_feasibility_and_get_cost(&instance, &best_solution).0;
    let mut incumbent_cost = check_feasibility_and_get_cost(&instance, &incumbent).0;

    for i in 1..9900 {
        new_solution = one_reinsert_probability(&incumbent, &instance);

        let (cost, feasible) = check_feasibility_and_get_cost(&instance, &new_solution);
        delta_e = cost as f64 - incumbent_cost as f64;

        let p = f64::consts::E.powf((-1.0 * delta_e) / temp);

        if feasible && delta_e < 0.0 {
            incumbent = new_solution;
            incumbent_cost = cost;

            if incumbent_cost < best_cost {
                best_solution = incumbent.clone();
                best_cost = incumbent_cost;
            }
        } else if feasible && rand::random::<f64>() < p {
            incumbent = new_solution;
            incumbent_cost = cost;
        }

        temp = temp * alpha;
    }

    return (best_solution, best_cost);
}


fn find_avg_delta(
    init_solution: &Vec<Vec<u32>>,
    instance: &Instance,
    prob: f64,
) -> (f64, Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut incumbent: Vec<Vec<u32>> = Vec::with_capacity(init_solution.len());
    let mut best_solution: Vec<Vec<u32>> = Vec::with_capacity(init_solution.len());
    incumbent.extend_from_slice(&init_solution);
    best_solution.extend_from_slice(&init_solution);
    let mut best_cost = check_feasibility_and_get_cost(&instance, &best_solution).0;
    let mut incumbent_cost = check_feasibility_and_get_cost(&instance, &incumbent).0;

    let mut new_solution: Vec<Vec<u32>>;
    let mut all_delta_e: Vec<f64> = Vec::new();
    let mut delta_e: f64;

    for _w in 1..=100 {
        new_solution = one_reinsert_probability(&init_solution, &instance);
        let (cost, feasible) = check_feasibility_and_get_cost(&instance, &new_solution);
        delta_e = incumbent_cost as f64 - cost as f64;

        if feasible && delta_e < 0.0 {
            incumbent = new_solution;
            incumbent_cost = cost;

            if incumbent_cost < best_cost {
                best_solution = incumbent.clone();
                best_cost = incumbent_cost;
            }
        } else if feasible {
            if rand::random::<f64>() < prob {
                incumbent = new_solution;
                incumbent_cost = cost;
            }
            all_delta_e.push(delta_e);
        }
    }

    return (
        all_delta_e.iter().sum::<f64>() / all_delta_e.len() as f64,
        incumbent,
        best_solution,
    );
}
