use std::collections::HashSet;
use simmulated_annealing::simmulated_annealing::OperatorFn;
use file_reader::parse_data::*;
use checker::checker::*;
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;
use local_search::operators::*;
use log::info;

pub fn alns_general(instance: &Instance, operators: &Vec<OperatorFn>) -> (Vec<Vec<u32>>, u128, Vec<Vec<f64>>) {
    let mut incumbent = get_init_solution(instance.num_calls, instance.num_vehicles);
    let mut incumbent_cost = check_feasibility_and_get_cost(&instance, &incumbent).0;
    let mut best_sol = incumbent.clone();
    let mut best_cost = incumbent_cost;
    let mut new_solution: Vec<Vec<u32>>;
    let mut new_solution_cost = incumbent_cost;
    let mut feasibility = true;
    let mut seen_solutions: HashSet<Vec<Vec<u32>>> = HashSet::new();

    let mut weights: Vec<f64> = vec![1.0 / operators.len() as f64; operators.len()];
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();

    let mut weights_history : Vec<Vec<f64>> = Vec::new();
    let mut best_history: Vec<u128> = Vec::new();


    let r = 0.3;
    let mut iterations_since_improvement = 0;
    let mut iterations = 0;
    let max_iterations = 10000;
    let escape_condition = 500;
    let mut escape_size = 20;
    let segment_size = 50;
    let mut operator_use_counts = vec![0; operators.len()];
    let mut operator_points = vec![0; operators.len()];

    while iterations < max_iterations {
        best_history.push(best_cost);
        weights_history.push(weights.clone());
        iterations += 1;

        // Escape if reached escape conditions
        if iterations_since_improvement > escape_condition {
            let operator = operators[dist.sample(&mut rng)];
            incumbent = escape(&instance, &incumbent, operator, best_cost, escape_size);
            escape_size += 1;
            (incumbent_cost, _) = check_feasibility_and_get_cost(&instance, &incumbent);

            if incumbent_cost < best_cost {
                best_cost = incumbent_cost;
                best_sol = incumbent.clone();
            }

            iterations_since_improvement = 0
        }

        new_solution = incumbent.clone();
        // Choose operator
        let operator_index = dist.sample(&mut rng);
        let operator = operators[operator_index];
        operator_use_counts[operator_index] += 1;

        // Apply operator
        new_solution = operator(&instance, &new_solution);
        (new_solution_cost, feasibility) = check_feasibility_and_get_cost(&instance, &new_solution);

        if feasibility {
            // Save best seen solution
            if new_solution_cost < best_cost {
                best_cost = new_solution_cost;
                best_sol = new_solution.clone();
                incumbent_cost = new_solution_cost;
                incumbent = new_solution.clone();
                iterations_since_improvement = 0;
                operator_points[operator_index] += 4;
            }

            // Acceptance criteria
            else if new_solution_cost < incumbent_cost || rng.random::<f64>() < (max_iterations as f64 - iterations as f64) / max_iterations as f64{
                incumbent_cost = new_solution_cost;
                incumbent = new_solution.clone();
                operator_points[operator_index] += 2;
                iterations_since_improvement += 1;
            }

            // Points for detecting a new solution
            else if !seen_solutions.contains(&new_solution) {
                seen_solutions.insert(new_solution.clone());
                operator_points[operator_index] += 1;
                iterations_since_improvement += 1;
            }
            seen_solutions.insert(new_solution.clone());
        }

        if iterations % segment_size == 0 {
            // Update weights and counts
            for weights_i in 0..weights.len() {
                // println!("{}", operator_points[weights_i]);
                // println!("{}", operator_use_counts[weights_i]);
                weights[weights_i] = f64::max(weights[weights_i] * (1.0 - r) + r * (operator_points[weights_i] as f64 / operator_use_counts[weights_i] as f64),0.05);
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
                },
                Err(_) => {panic!("After updating weights in alns, creating weighted index failed. Weights: {:?}", weights);}
            }
        }
    }

    for c in best_history {
        print!("{},",c);
    }
    (best_sol, best_cost, weights_history)
}

fn escape(instance: &Instance, solution: &Vec<Vec<u32>>, operator: OperatorFn, best_cost: u128, escape_iterations: usize) -> Vec<Vec<u32>> {
    let mut end_solution = solution.clone();

    for _i in 0..escape_iterations {
        let new = actual_k_reinsert(&instance, &end_solution);
        let (cost, feasibility) = check_feasibility_and_get_cost(&instance, &new);
        if !feasibility {continue;}

        end_solution = new;

        if cost < best_cost {
            return end_solution;
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