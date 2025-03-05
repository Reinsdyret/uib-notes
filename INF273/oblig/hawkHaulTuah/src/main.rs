use checker::checker::*;
use file_reader::parse_data::*; // Import read_file function
use local_search::{local_search::*, operators};
use log::{debug, error, info, log_enabled, warn, Level};
use random_meta::random::*;
use rayon::prelude::*;
use simmulated_annealing::simmulated_annealing::*;
use std::time::Instant;
use std::{u128, u32};
use std::time::Duration;
use local_search::operators::{one_reinsert_greedy_insert, reinsert_sub_route, two_call_swap};

fn main() {
    let filenames: Vec<&str> = vec![
        "src/data/Call_7_Vehicle_3.txt",
        "src/data/Call_18_Vehicle_5.txt",
        "src/data/Call_35_Vehicle_7.txt",
        "src/data/Call_80_Vehicle_20.txt",
        "src/data/Call_130_Vehicle_40.txt",
        "src/data/Call_300_Vehicle_90.txt",
    ];
    env_logger::init();
    info!("STARTEd");
    warn!("Start");

    // for filename in filenames {
    //     run_simmulated_annealing_report(filename, true, 0.8, 0.1);
    // }

    
    // for filename in filenames {
    //     tune_weights(filename, 0.1);
    // }
    //
    let my_operators = vec![
        (reinsert_sub_route as OperatorFn, "reinsert_sub_route"),
        (one_reinsert_greedy_insert as OperatorFn, "one_reinsert_greedy_insert"),
        (two_call_swap as OperatorFn, "two_call_swap_extended")
    ];
    let op_weights = vec![0.2, 0.5, 0.3];
    for filename in filenames {
        run_simmulated_annealing_report_with_operators_and_weights(
            filename, true, 0.8, 0.1, &my_operators, &op_weights
        );
    }
    
    // for filename in filenames {
    //     run_local_search_report(filename, true);
    // }
    // for filename in filenames {
    //     run_random_report(filename);
    // }
}

fn run_simmulated_annealing_report(filename: &str, parallel: bool, prob: f64, t_final: f64) {
    let instance = read_file(filename);

    let outsource_sol = get_init_solution(instance.num_calls, instance.num_vehicles);

    let results: Vec<(Vec<Vec<u32>>, u128)>;

    let now = Instant::now();

    // Run with default operators and weights
    if parallel {
        results = (0..10)
            .into_par_iter()
            .map(|_| run_sa(&outsource_sol, &instance, prob, t_final))
            .collect();
    } else {
        results = (0..10)
            .map(|_| run_sa(&outsource_sol, &instance, prob, t_final))
            .collect();
    }

    let total_time = Instant::now().duration_since(now).as_millis();
    let total_sum: u128 = results.iter().map(|(_, cost)| *cost).sum();

    let (best_solution, best_cost) = results.iter().min_by_key(|(_, cost)| *cost).unwrap();
    let init_cost = check_feasibility_and_get_cost(&instance, &outsource_sol).0;
    let avg_cost = total_sum / 10;
    let diff_avg = init_cost - avg_cost;
    let improvement_avg: f64 = (diff_avg as f64 / init_cost as f64) * 100.0;
    let diff_best = init_cost - best_cost;
    let improvement_best: f64 = (diff_best as f64 / init_cost as f64) * 100.0;

    println!(
        "Ran simulated annealing with default operators. {filename}
    Avg time taken: {}ms
    Best cost: {}
    Avg cost: {}
    Improvement avg: {}%
    Improvement best: {}%
    Solution: {:?}",
        total_time / 10,
        best_cost,
        avg_cost,
        improvement_avg,
        improvement_best,
        concat_solution(&best_solution)
    );
}

fn run_simmulated_annealing_report_with_weights(filename: &str, parallel: bool, prob: f64, t_final: f64, weights: &[f64]) {
    let operators = get_available_operators();
    run_simmulated_annealing_report_with_operators_and_weights(filename, parallel, prob, t_final, &operators, weights);
}
fn run_simmulated_annealing_report_with_operators_and_weights(
    filename: &str, 
    parallel: bool, 
    prob: f64, 
    t_final: f64, 
    operators: &[(OperatorFn, &'static str)],
    weights: &[f64]
) {
    let instance = read_file(filename);
    let outsource_sol = get_init_solution(instance.num_calls, instance.num_vehicles);
    
    // Make sure weights match the number of operators
    assert_eq!(operators.len(), weights.len(), 
        "Number of weights ({}) must match number of operators ({})", weights.len(), operators.len());
    
    let results: Vec<(Vec<Vec<u32>>, u128)>;
    let now = Instant::now();

    if parallel {
        results = (0..10)
            .into_par_iter()
            .map(|_| run_sa_with_operators(&outsource_sol, &instance, prob, t_final, operators, weights))
            .collect();
    } else {
        results = (0..10)
            .map(|_| run_sa_with_operators(&outsource_sol, &instance, prob, t_final, operators, weights))
            .collect();
    }

    let total_time = Instant::now().duration_since(now).as_millis();
    let total_sum: u128 = results.iter().map(|(_, cost)| *cost).sum();

    let (best_solution, best_cost) = results.iter().min_by_key(|(_, cost)| *cost).unwrap();
    let init_cost = check_feasibility_and_get_cost(&instance, &outsource_sol).0;
    let avg_cost = total_sum / 10;
    let diff_avg = init_cost - avg_cost;
    let improvement_avg: f64 = (diff_avg as f64 / init_cost as f64) * 100.0;
    let diff_best = init_cost - best_cost;
    let improvement_best: f64 = (diff_best as f64 / init_cost as f64) * 100.0;

    // Format the weights for display
    let weight_display: Vec<String> = operators.iter().zip(weights.iter())
        .map(|((_, name), weight)| format!("{}: {:.2}", name, weight))
        .collect();
    
    println!(
        "Ran simulated annealing with custom operators and weights [{}]. {filename}
    Avg time taken: {}ms
    Best cost: {}
    Avg cost: {}
    Improvement avg: {}%
    Improvement best: {}%
    Solution: {:?}",
        weight_display.join(", "),
        total_time / 10,
        best_cost,
        avg_cost,
        improvement_avg,
        improvement_best,
        concat_solution(&best_solution)
    );
}

fn run_local_search_report(filename: &str, parallel: bool) {
    let instance = read_file(filename);

    let outsource_sol = get_init_solution(instance.num_calls, instance.num_vehicles);
    let results: Vec<(Vec<Vec<u32>>, u128)>;

    let now = Instant::now();

    if parallel {
        results = (0..10)
            .into_par_iter()
            .map(|_| run_local_search(&outsource_sol, Operator::OneReinsert, &instance))
            .collect();
    } else {
        results = (0..10)
            .into_par_iter()
            .map(|_| run_local_search(&outsource_sol, Operator::OneReinsert, &instance))
            .collect();
    }

    let total_time = Instant::now().duration_since(now).as_millis();

    let total_sum: u128 = results.iter().map(|(_, cost)| cost).sum();
    let (best_solution, best_cost) = results.iter().min_by_key(|(_, cost)| *cost).unwrap();
    let init_cost = check_feasibility_and_get_cost(&instance, &outsource_sol).0;
    let avg_cost = total_sum / 10;
    let diff_avg = init_cost - avg_cost;
    let improvement_avg: f64 = (diff_avg as f64 / init_cost as f64) * 100.0;
    let diff_best = init_cost - best_cost;
    let improvement_best: f64 = (diff_best as f64 / init_cost as f64) * 100.0;

    println!(
        "Ran simmulated_annealing with 1-reinsert. {filename}
    Avg time taken: {}ms
    Best cost: {}
    Avg cost: {}
    Improvement avg: {}%
    Improvement best: {}%
    Solution: {:?}",
        total_time / 10,
        best_cost,
        avg_cost,
        improvement_avg,
        improvement_best,
        concat_solution(&best_solution)
    );
}

fn run_random_report(filename: &str) {
    let instance = read_file(filename);

    let outsource_sol = get_init_solution(instance.num_calls, instance.num_vehicles);

    let (init_cost, feasible) = check_feasibility_and_get_cost(&instance, &outsource_sol);
    assert!(feasible);

    let results: Vec<(u128, Vec<Vec<u32>>)>;

    let now = Instant::now();

    results = (0..10)
        .map(|_| run_random(&instance))
        .filter(|(cost, sol)| sol.len() > 0 && *cost != u128::MAX)
        .collect();

    let total_time = Instant::now().duration_since(now).as_millis();

    if results.len() == 0 {
        println!(
            "\nNo solution found 
            Time taken on avg: {}ms",
            total_time / 10
        );
        return;
    }

    let found_feasible = results.len() as u128;
    let best = results.iter().min_by_key(|(cost, _)| *cost).unwrap();
    let best_cost = best.0;
    let best_solution = best.1.clone();
    let total_sum: u128 = results.iter().map(|(cost, _)| *cost).sum();

    let diff = init_cost as f64 - best_cost as f64;
    let improvement = (diff / init_cost as f64) * 100.0;

    println!("Best cost: {best_cost}");
    println!("Best solution: {0:?}", concat_solution(&best_solution));
    println!("Average time for 10k: {}ms", { total_time / 10 });
    if found_feasible > 0 {
        println!("Average cost: {}", total_sum / found_feasible);
    }
    println!("Outsource cost: {init_cost}");
    println!("Improvement: {improvement}%");
}

fn tune_weights(filename: &str, step_size: f64) {
    println!("\n=========== WEIGHT TUNING EXPERIMENT ===========");
    println!("Testing weight configurations with step size {} on {}", step_size, filename);
    
    let operators = get_available_operators();
    let num_operators = operators.len();
    assert_eq!(num_operators, 3, "This function is designed for 3 operators");
    
    let mut results: Vec<(Vec<f64>, u128)> = Vec::new();
    let instance = read_file(filename);
    let outsource_sol = get_init_solution(instance.num_calls, instance.num_vehicles);
    
    // Generate weight combinations
    let mut w1: f64 = 0.0;
    while w1 <= 1.0 {
        let mut w2: f64 = 0.0;
        while w2 <= 1.0 - w1 {
            // The third weight is determined by the first two
            let w3: f64 = 1.0 - w1 - w2;
            
            // Skip if weights don't sum to approximately 1.0
            if (w1 + w2 + w3 - 1.0).abs() > 0.001 {
                w2 += step_size;
                continue;
            }
            
            let weights = vec![w1, w2, w3];
            
            // Display the weight configuration
            let weight_display: Vec<String> = operators.iter().zip(weights.iter())
                .map(|((_, name), weight)| format!("{}: {:.2}", name, weight))
                .collect();
            println!("\nTesting weights: [{}]", weight_display.join(", "));
            
            // Run SA with these weights
            let (_, cost) = run_sa_with_operators(
                &outsource_sol, 
                &instance,
                0.8,  // prob
                0.1,  // t_final
                &operators,
                &weights
            );
            
            println!("  Cost: {}", cost);
            results.push((weights, cost));
            
            w2 += step_size;
        }
        w1 += step_size;
    }
    
    // Sort results by cost and display them
    results.sort_by_key(|(_, cost)| *cost);
    
    println!("\n======= WEIGHT TUNING RESULTS (TOP 10) =======");
    for (i, (weights, cost)) in results.iter().take(10).enumerate() {
        let weight_display: Vec<String> = operators.iter().zip(weights.iter())
            .map(|((_, name), weight)| format!("{}: {:.2}", name, weight))
            .collect();
        println!("{}. Cost: {}, Weights: [{}]", i+1, cost, weight_display.join(", "));
    }
    
    // Recommend the best configuration
    println!("\nRECOMMENDED WEIGHT CONFIGURATION:");
    let best_weights = &results[0].0;
    let weight_display: Vec<String> = operators.iter().zip(best_weights.iter())
        .map(|((_, name), weight)| format!("{}: {:.2}", name, weight))
        .collect();
    println!("Weights: [{}]", weight_display.join(", "));
    println!("Best cost: {}", results[0].1);
}

fn concat_solution(solution: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut res = Vec::new();

    for (i, route) in solution.into_iter().enumerate() {
        res.extend(route);

        if i != solution.len() - 1 {
            res.push(0);
        }
    }

    return res;
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

    return outsource_sol;
}
