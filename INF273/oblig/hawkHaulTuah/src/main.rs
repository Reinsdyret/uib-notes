use checker::checker::*;
use file_reader::parse_data::*; // Import read_file function
use local_search::{local_search::*, operators};
use random_meta::random::*;
use rayon::prelude::*;
use simmulated_annealing::simmulated_annealing::*;
use std::time::Instant;
use std::{u128, u32};

fn main() {
    let filenames: Vec<&str> = vec![
        "src/data/Call_7_Vehicle_3.txt",
        "src/data/Call_18_Vehicle_5.txt",
        "src/data/Call_35_Vehicle_7.txt",
        "src/data/Call_80_Vehicle_20.txt",
        "src/data/Call_130_Vehicle_40.txt",
        "src/data/Call_300_Vehicle_90.txt",
    ];

    // run_random_report(filename);
    // run_local_search_report(filename);
    //for filename in filenames {
    //    run_random_report(filename);
    //}
    //for filename in filenames {
    //    run_simmulated_annealing_report(filename, true, 0.8, 0.1);
    //}
    for filename in filenames {
        run_local_search_report(filename, true);
    }
}

fn run_simmulated_annealing_report(filename: &str, parallel: bool, prob: f64, t_final: f64) {
    let instance = read_file(filename);

    let outsource_sol = get_init_solution(instance.num_calls, instance.num_vehicles);

    let results: Vec<(Vec<Vec<u32>>, u128)>;

    let now = Instant::now();

    // Run with 1-reinsert
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
