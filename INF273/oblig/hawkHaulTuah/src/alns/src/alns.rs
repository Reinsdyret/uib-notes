use checker::checker::*;
use file_reader::parse_data::*;
use local_search::operators::*;
use log::info;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use simmulated_annealing::simmulated_annealing::{OperatorFn, find_avg_delta_with_operators};
use std::collections::HashSet;
use rand::random;

pub fn alns_general(
    instance: &Instance,
    operators: &Vec<OperatorFn>,
) -> (Vec<Vec<u32>>, u128, Vec<Vec<f64>>) {
    let mut incumbent = get_init_solution(instance.num_calls, instance.num_vehicles);
    let mut incumbent_cost = check_feasibility_and_get_cost(&instance, &incumbent).0;
    let mut best_sol = incumbent.clone();
    let mut best_cost = incumbent_cost;
    let mut new_solution: Vec<Vec<u32>>;
    let mut new_solution_cost;
    let mut feasibility;
    let delta_e_avg: f64;
    let mut seen_solutions: HashSet<Vec<Vec<u32>>> = HashSet::new();

    let mut weights: Vec<f64> = vec![1.0 / operators.len() as f64; operators.len()];
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();

    let mut weights_history: Vec<Vec<f64>> = Vec::new();
    let mut best_history: Vec<u128> = Vec::new();
    let mut incumbent_history: Vec<u128> = Vec::new();

    let mut r = 0.1;
    let mut iterations_since_improvement = 0;
    let mut iterations = 0;
    let max_iterations = 24900;
    let escape_condition = 200;
    let mut escape_size = 5;
    let segment_size = 300;
    let mut operator_use_counts = vec![0; operators.len()];
    let mut operator_points = vec![0; operators.len()];

    // Warmup or find avg delta for temperature for sa like acceptance criteria
    (delta_e_avg, incumbent, best_sol) = find_avg_delta_with_operators(&incumbent, &instance, 0.8, &operators, &dist);
    let t_zero = (-1.0 * delta_e_avg) / (0.8f64).ln();
    let t_final = 0.5;
    let alpha = f64::powf(t_final / t_zero, 1.0 / 24900.0);
    let mut temp = t_zero;
    let mut p: f64 = 0.9;
    let mut delta_e: f64;

    while iterations < max_iterations {
        r = 0.3 * (1.0 - iterations as f64 / max_iterations as f64);
        best_history.push(best_cost);
        weights_history.push(weights.clone());
        incumbent_history.push(incumbent_cost);
        iterations += 1;

        if iterations_since_improvement > escape_condition * 3 {
            incumbent = best_sol.clone();
            incumbent_cost = best_cost;
            iterations_since_improvement = 0;
        }

        // Escape if reached escape conditions
        if iterations_since_improvement >= escape_condition {
            let operator = operators[dist.sample(&mut rng)];
            incumbent = escape(&instance, &incumbent, &operator, best_cost, escape_size);
            // escape_size += 1;
            (incumbent_cost, _) = check_feasibility_and_get_cost(&instance, &incumbent);

            if incumbent_cost < best_cost {
                best_cost = incumbent_cost;
                best_sol = incumbent.clone();
                iterations_since_improvement = 0
            } else {
                // escape_size = (escape_size as f64 * 1.5) as usize;
            }
        }

        new_solution = incumbent.clone();
        // Choose operator
        let operator_index = dist.sample(&mut rng);
        let operator = operators[operator_index];
        operator_use_counts[operator_index] += 1;


        // Apply operator
        new_solution = operator(&instance, &new_solution);
        (new_solution_cost, feasibility) = check_feasibility_and_get_cost(&instance, &new_solution);
        delta_e = new_solution_cost as f64 - incumbent_cost as f64;

        p = std::f64::consts::E.powf((-1.0 * delta_e) / temp);

        if feasibility {
            if !seen_solutions.contains(&new_solution) {
                seen_solutions.insert(new_solution.clone());
                operator_points[operator_index] += 1;
            }

            if delta_e < 0.0 {
                iterations_since_improvement = 0;

                incumbent = new_solution;
                incumbent_cost = new_solution_cost;

                if incumbent_cost < best_cost {
                    // Significant improvement - new best solution
                    let improvement = best_cost - incumbent_cost;
                    let improvement_percentage = improvement as f64 / best_cost as f64;

                    // Scale points based on improvement size
                    if improvement_percentage > 0.05 { // >5% improvement
                        operator_points[operator_index] += 10;
                    } else if improvement_percentage > 0.01 { // >1% improvement
                        operator_points[operator_index] += 6;
                    } else {
                        operator_points[operator_index] += 4; // Minor improvement
                    }

                    best_cost = incumbent_cost;
                    best_sol = incumbent.clone();
                } else {
                    operator_points[operator_index] += 2;
                }
            } else if random::<f64>() < p {
                incumbent = new_solution;
                incumbent_cost = new_solution_cost;
            }
        }

        temp = temp * alpha;

        if iterations % segment_size == 0 {
            // Update weights and counts
            for weights_i in 0..weights.len() {
                // println!("{}", operator_points[weights_i]);
                // println!("{}", operator_use_counts[weights_i]);
                weights[weights_i] = f64::max(
                    weights[weights_i] * (1.0 - r)
                        + r * (operator_points[weights_i] as f64
                            / operator_use_counts[weights_i] as f64),
                    0.05,
                );
            }

            // Normalize weights so they sum to 1.0
            let sum: f64 = weights.iter().sum();
            for weights_i in 0..weights.len() {
                weights[weights_i] = weights[weights_i] / sum;
            }

            operator_points = vec![0; operator_points.len()];
            operator_use_counts = vec![0; operator_use_counts.len()];

            match WeightedIndex::new(&weights) {
                Ok(weighted_index) => {
                    dist = weighted_index;
                    // println!("{:?}", weights);
                }
                Err(_) => {
                    panic!("After updating weights in alns, creating weighted index failed. Weights: {:?}", weights);
                }
            }
        }
    }

    // for c in best_history {
    //     print!("{},", c);
    // }
    // println!();
    // for c in incumbent_history {
    //     print!("{},", c);
    // }
    (best_sol, best_cost, weights_history)
}

fn escape(
    instance: &Instance,
    solution: &Vec<Vec<u32>>,
    operator: &OperatorFn,
    best_cost: u128,
    escape_iterations: usize,
) -> Vec<Vec<u32>> {
    let mut end_solution = solution.clone();

    for _i in 0..escape_iterations {
        let new = random_removal_first_feasible_insert(&instance, &end_solution);
        let (cost, feasibility) = check_feasibility_and_get_cost(&instance, &new);
        if !feasibility {
            continue;
        }

        end_solution = new;

        if cost < best_cost {
            break;
        }
    }

    end_solution
}

fn get_init_solution(num_calls: u32, num_vehicles: u32) -> Vec<Vec<u32>> {
    let mut outsource_sol: Vec<Vec<u32>> = vec![];
    for _i in 0..num_vehicles - 1 {
        outsource_sol.push(Vec::new());
    }

    let mut outsource_truck: Vec<u32> = Vec::new();

    for i in 1..num_calls + 1 {
        outsource_truck.push(i);
        outsource_truck.push(i);
    }

    outsource_sol.push(outsource_truck);

    outsource_sol
}
