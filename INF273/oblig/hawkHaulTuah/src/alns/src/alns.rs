use solution::solution::*;
use operators::*;
use checker::checker::*;
use file_reader::parse_data::*;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use simmulated_annealing::simmulated_annealing::{OperatorFn, find_avg_delta_with_operators};
use std::collections::HashSet;
use local_search::operators::*;
use rand::{random};

pub fn alns_general(
    instance: &Instance,
    operators: &Vec<OperatorFn>,
) -> (Vec<Vec<u32>>, u128, Vec<Vec<f64>>, Vec<u128>, Vec<u128>, Vec<Vec<f64>>) {
    let mut incumbent = get_init_solution(instance.num_calls, instance.num_vehicles);
    let mut incumbent_cost = check_feasibility_and_get_cost(&instance, &incumbent).0;
    let mut best_sol = incumbent.clone();
    let mut best_cost = incumbent_cost;
    let mut new_solution: Vec<Vec<u32>>;
    let mut new_solution_cost;
    let mut feasibility;
    let mut seen_solutions: HashSet<Vec<Vec<u32>>> = HashSet::new();

    let mut weights: Vec<f64> = vec![1.0 / operators.len() as f64; operators.len()];
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();

    // Track histories
    let mut weights_history: Vec<Vec<f64>> = Vec::with_capacity(24900);
    let mut best_history: Vec<u128> = Vec::with_capacity(24900);
    let mut incumbent_history: Vec<u128> = Vec::with_capacity(24900);
    
    // New tracking data
    let mut operator_delta_costs: Vec<Vec<f64>> = Vec::new();

    let mut d = 0.2;
    let mut r = 0.2;
    let mut iterations_since_improvement = 0;
    let mut iterations = 0;
    let max_iterations = 25000;
    let escape_condition = 500;
    let mut escape_size = 1;
    let segment_size = 100;
    
    let mut operator_use_counts = vec![0; operators.len()];
    let mut operator_points = vec![0; operators.len()];

    let mut delta_e: f64;
    
    // Initialize tracking for the first iteration
    operator_delta_costs.push(vec![0.0; operators.len()]);

    while iterations < max_iterations {
        best_history.push(best_cost);
        weights_history.push(weights.clone());
        incumbent_history.push(incumbent_cost);
        iterations += 1;
        iterations_since_improvement += 1;
        d = 0.2 * ((max_iterations - iterations) as f64 / max_iterations as f64) * best_cost as f64;


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
                iterations_since_improvement = 0;
            } else {
                escape_size = (escape_size as f64 * 1.5) as usize;
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
        
        // Store delta cost for this operator at this iteration
        let mut deltas = vec![0.0; operators.len()];
        deltas[operator_index] = delta_e;
        operator_delta_costs.push(deltas);

        if feasibility {
            /*
            if !seen_solutions.contains(&new_solution) {
                seen_solutions.insert(new_solution.clone());
                operator_points[operator_index] += 1;
            }*/

            if delta_e < 0.0 {
                incumbent = new_solution;
                incumbent_cost = new_solution_cost;

                if incumbent_cost < best_cost {
                    iterations_since_improvement = 0;

                    // Significant improvement - new best solution
                    let improvement = best_cost - incumbent_cost;
                    let improvement_percentage = improvement as f64 / best_cost as f64;
                    
                    // Scale points based on improvement size
                    /*
                    if improvement_percentage > 0.05 { // >5% improvement
                        operator_points[operator_index] += 10;
                    } else if improvement_percentage > 0.01 { // >1% improvement
                        operator_points[operator_index] += 6;
                    } else {
                        operator_points[operator_index] += 4; // Minor improvement
                    }
                    */
                    operator_points[operator_index] += 5;

                    best_cost = incumbent_cost;
                    best_sol = incumbent.clone();
                } else {
                    operator_points[operator_index] += 3;
                }
            } else if (incumbent_cost as f64) < (best_cost as f64 * d) {
                incumbent = new_solution;
                incumbent_cost = new_solution_cost;
                operator_points[operator_index] += 1;
            }
        }

        if iterations % segment_size == 0 {
            let sum_points: i32 = operator_points.iter().sum();
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

    // Return histories along with the solution
    (best_sol, best_cost, weights_history, best_history, incumbent_history, 
     operator_delta_costs)
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
        let new = one_reinsert_probability(&instance, &end_solution);
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

/// Temperature cooling function for the ALNS algorithm
/// Returns the new temperature based on the current iteration
fn cool(
    current_iteration: usize,
    max_iterations: usize,
    t_zero: f64,
    t_final: f64
) -> f64 {
    // Calculate progress as a value between 0 and 1
    let progress = current_iteration as f64 / max_iterations as f64;
    
    // More aggressive exponential cooling schedule
    // Start high but cool continuously throughout the process
    
    // Use a modified exponential cooling that cools faster
    // This gives a good balance of exploration and exploitation
    let cooling_exponent = 10.0; // Higher value = faster cooling
    let normalized_temp = f64::exp(-cooling_exponent * progress);
    
    // Scale between initial and final temperature
    let temp = t_final + (t_zero - t_final) * normalized_temp;
    
    // Ensure we don't go below t_final
    f64::max(temp, t_final)
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
