use std::f64;

use checker::checker::*;
use file_reader::parse_data::Instance;
use local_search::operators::*;
use rand::{random, thread_rng, Rng};
use rand::distributions::{Distribution, WeightedIndex};

// Function type for our operators
pub type OperatorFn = fn(&Instance, &Vec<Vec<u32>>) -> Vec<Vec<u32>>;

// Define the available operators
pub fn get_available_operators() -> Vec<(OperatorFn, &'static str)> {
    vec![
        (reinsert_sub_route as OperatorFn, "reinsert_sub_route"),
        (one_reinsert_greedy_insert as OperatorFn, "one_reinsert_greedy_insert"),
        (two_call_swap as OperatorFn, "two_call_swap_extended"),
    ]
}

// Default weights if none are provided
pub fn get_default_weights() -> Vec<f64> {
    vec![0.33,0.33,0.33]
}

pub fn run_sa(
    init_solution: &Vec<Vec<u32>>,
    instance: &Instance,
    prob: f64,
    t_final: f64,
) -> (Vec<Vec<u32>>, u128) {
    // Use default operators and weights
    let operators = get_available_operators();
    let weights = get_default_weights();
    
    run_sa_with_operators(init_solution, instance, prob, t_final, &operators, &weights)
}

pub fn run_sa_with_operators(
    init_solution: &Vec<Vec<u32>>,
    instance: &Instance,
    prob: f64,
    t_final: f64,
    operators: &[(OperatorFn, &'static str)],
    weights: &[f64],
) -> (Vec<Vec<u32>>, u128) {
    // Verify that operators and weights have the same length
    assert_eq!(operators.len(), weights.len(), 
               "Number of operators must match number of weights");
    
    // Create weighted distribution for operator selection
    let weight_dist = match WeightedIndex::new(weights) {
        Ok(dist) => dist,
        Err(_) => {
            // Fallback if weights are invalid
            panic!("Invalid weights provided for operators");
        }
    };
    
    // Run warmup phase with the provided operators
    let (delta_avg, mut incumbent, mut best_solution) =
        find_avg_delta_with_operators(init_solution, instance, prob, operators, &weight_dist);

    let t_zero = (-1.0 * delta_avg) / (0.8f64).ln();
    let alpha = f64::powf(t_final / t_zero, 1.0 / 9900.0);
    let mut temp = t_zero;

    let mut new_solution: Vec<Vec<u32>>;
    let mut delta_e: f64;
    let mut best_cost = check_feasibility_and_get_cost(&instance, &best_solution).0;
    let mut incumbent_cost = check_feasibility_and_get_cost(&instance, &incumbent).0;
    let mut p: f64 = 0.9;
    
    let mut rng = rand::thread_rng();

    for i in 1..9900 {
        // Select operator based on weights
        let op_idx = weight_dist.sample(&mut rng);
        let operator = operators[op_idx].0;
        
        // Apply the selected operator
        new_solution = operator(&instance, &incumbent);

        // Evaluate the new solution
        let (cost, feasible) = check_feasibility_and_get_cost(&instance, &new_solution);
        delta_e = cost as f64 - incumbent_cost as f64;

        p = f64::consts::E.powf((-1.0 * delta_e) / temp);

        if feasible && delta_e < 0.0 {
            // Always accept improving moves
            incumbent = new_solution;
            incumbent_cost = cost;

            if incumbent_cost < best_cost {
                best_solution = incumbent.clone();
                best_cost = incumbent_cost;
            }
        } else if feasible && rand::random::<f64>() < p {
            // Probabilistic acceptance of non-improving moves
            incumbent = new_solution;
            incumbent_cost = cost;
        }

        temp = temp * alpha;
    }

    (best_solution, best_cost)
}

fn find_avg_delta(
    init_solution: &Vec<Vec<u32>>,
    instance: &Instance,
    prob: f64,
) -> (f64, Vec<Vec<u32>>, Vec<Vec<u32>>) {
    // Use default operators and weights
    let operators = get_available_operators();
    let weights = get_default_weights();
    
    // Create weighted distribution for operator selection
    let weight_dist = match WeightedIndex::new(&weights) {
        Ok(dist) => dist,
        Err(_) => {
            // Fallback if weights are invalid
            panic!("Invalid weights provided for default operators");
        }
    };
    
    find_avg_delta_with_operators(init_solution, instance, prob, &operators, &weight_dist)
}

fn find_avg_delta_with_operators(
    init_solution: &Vec<Vec<u32>>,
    instance: &Instance,
    prob: f64,
    operators: &[(OperatorFn, &'static str)],
    weight_dist: &WeightedIndex<f64>,
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
    let mut rng = rand::thread_rng();

    for _w in 1..=100 {
        // Select operator based on weights
        let op_idx = weight_dist.sample(&mut rng);
        let operator = operators[op_idx].0;
        
        // Apply the selected operator
        new_solution = operator(&instance, &incumbent);
        
        // Evaluate the new solution
        let (cost, feasible) = check_feasibility_and_get_cost(&instance, &new_solution);
        delta_e = incumbent_cost as f64 - cost as f64;

        if feasible && delta_e < 0.0 {
            // Accept improving solutions
            incumbent = new_solution;
            incumbent_cost = cost;

            if incumbent_cost < best_cost {
                best_solution = incumbent.clone();
                best_cost = incumbent_cost;
            }
        } else if feasible {
            // Probabilistically accept non-improving solutions
            if rand::random::<f64>() < prob {
                incumbent = new_solution;
                incumbent_cost = cost;
            }
            all_delta_e.push(delta_e);
        }
    }

    // Return average delta, incumbent solution, and best solution
    if all_delta_e.is_empty() {
        // If no non-improving moves were found, use a default value
        return (0.1, incumbent, best_solution);
    }
    
    (
        all_delta_e.iter().sum::<f64>() / all_delta_e.len() as f64,
        incumbent,
        best_solution,
    )
}
