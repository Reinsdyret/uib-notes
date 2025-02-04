
use core::time;
use std::collections::HashMap;
use std::u32;
use std::time::Instant;
use file_reader::parse_data::*;  // Import read_file function
use checker::checker::*;
use random_meta::random::*;
use local_search::{local_search::*, operators};

fn main() {
    // let solution: Vec<u32> = vec![0, 2, 2, 0, 1, 5, 5, 3, 1, 3, 0, 7, 4, 6, 7, 4, 6];
    //let solution: Vec<u32> = vec![0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7];
    let filenames: Vec<&str> = vec!["src/data/Call_7_Vehicle_3.txt","src/data/Call_18_Vehicle_5.txt","src/data/Call_35_Vehicle_7.txt","src/data/Call_80_Vehicle_20.txt","src/data/Call_130_Vehicle_40.txt","src/data/Call_300_Vehicle_90.txt"];
    
    //let filename = "src/data/Call_7_Vehicle_3.txt"; 
    // let filename = "src/data/Call_18_Vehicle_5.txt";
    // let filename = "src/data/Call_35_Vehicle_7.txt";
    // let filename = "src/data/Call_80_Vehicle_20.txt";
    // let filename = "src/data/Call_130_Vehicle_40.txt";
    //let filename = "src/data/Call_300_Vehicle_90.txt";

    
    //run_local_search_report(filename);
    for filename in filenames { run_random_report(filename); }
}


fn run_local_search_report(filename: &str) {
    let instance = read_file(filename);

    let mut outsource_sol:Vec<Vec<u32>> = vec![];
    for i in 0 .. instance.num_vehicles {
        outsource_sol.push(Vec::new());
    }
    let mut outsource_truck: Vec<u32> = Vec::new();
    for i in 1 .. instance.num_calls + 1 {
        outsource_truck.push(i);
        outsource_truck.push(i);
    }
    outsource_sol.push(outsource_truck);

    
    // Run with 1-reinsert
    let mut best_cost: u128 = u128::MAX;
    let mut best_solution : Vec<Vec<u32>> = Vec::new();
    let mut total_time: u128 = 0;
    let mut total_sum: u128 = 0;

    let found_feasible = 10;
    
    for i in 0 .. 10 {
        println!("{}%", i);
        let now = Instant::now();
        let (solution, cost) = run_local_search(&outsource_sol, Operator::OneReinsert, &instance);
        let time_taken = Instant::now().duration_since(now).as_millis();
        total_time += time_taken;
        total_sum += cost;

        if cost < best_cost {
            best_cost = cost;
            best_solution = solution;
        }
    }
    

    println!("Ran 1-reinsert.
    Avg time taken: {}ms
    Best cost: {}
    Avg cost: {}
    Solution: {:?}",total_time / found_feasible, best_cost, total_sum / found_feasible, best_solution);

    // Run with 2-exchange


    // Run with 3-exchange
}


fn run_random_report(filename: &str) {
    let mut instance = read_file(filename);

    let mut outsource_sol:Vec<Vec<u32>> = vec![];
    for i in 0 .. instance.num_vehicles {
        outsource_sol.push(Vec::new());
    }

    let mut outsource_truck: Vec<u32> = Vec::new();

    for i in 1 .. instance.num_calls + 1 {
        outsource_truck.push(i);
        outsource_truck.push(i);
    }

    outsource_sol.push(outsource_truck);


    let (init_cost, feasible) = check_feasibility_and_get_cost(&instance, &outsource_sol);
    assert!(feasible);
    
    let mut best_cost: u128 = u128::MAX;
    let mut best_solution : Vec<Vec<u32>> = Vec::new();
    let mut total_time: u128 = 0;
    let mut total_sum: u128 = 0;

    let mut found_feasible = 0;


    for i in 1 ..=10 {
        println!("{}%", i * 10);
        let now = Instant::now(); 
        let (cost, solution) = run_random_10k(&mut instance);
        let time_taken = Instant::now().duration_since(now).as_millis();
        total_time += time_taken;
        
        // if no feasible solution found
        if solution.is_empty() {
            continue;
        }

        found_feasible += 1;

        total_sum += cost;
        
        if cost < best_cost {
            best_cost = cost;
            best_solution = solution;
        }
    }

    let diff = init_cost as f64 - best_cost as f64;
    let improvement = (diff / init_cost as f64) * 100.0;




    println!("Best cost: {best_cost}");
    println!("Best solution: {0:?}", concat_solution(&best_solution));
    println!("Average time for 10k: {}ms", {total_time / 10});
    if found_feasible > 0 { println!("Average cost: {}", total_sum / found_feasible); }
    println!("Outsource cost: {init_cost}");
    println!("Improvement: {improvement}%");

}
fn run_random_10k(instance: &mut Instance) -> (u128, Vec<Vec<u32>>) {
    let mut best_cost: u128 = u128::MAX;
    let mut best_solution : Vec<Vec<u32>> = Vec::new();
    let mut solution: Vec<Vec<u32>>;
    let mut cost: u128;
    let mut feasible: bool;
    let mut sum_time_random: u128 = 0;
    let mut sum_time_check: u128 = 0;

    for i in 0 .. 10_000 {
        // let now = Instant::now();
        solution = get_random_sol(instance.num_calls, instance.num_vehicles + 1, &instance.valid_vehicles);
        // let after = Instant::now().duration_since(now).as_micros();
        // sum_time_random += after;

        // let now = Instant::now();
        (cost, feasible) = check_feasibility_and_get_cost(&instance, &solution);
        // let after = Instant::now().duration_since(now).as_micros();
        // sum_time_check += after;

        if !feasible { continue; }

        if cost < best_cost {
            // println!("{solution:?}");
            best_cost = cost;
            best_solution = solution;
        }
    }

    // println!("Total time spent generating random: {}", sum_time_random);
    // println!("Total time spent checking solution: {}", sum_time_check);

    return (best_cost, best_solution);
}


fn concat_solution(solution: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut res = Vec::new();

    for (i, route) in solution.into_iter().enumerate() {
        res.extend(route);
        
        if i != solution.len() - 1 { res.push(0); }
    }

    return res;
}

