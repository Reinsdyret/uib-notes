
use core::time;
use std::u32;
use std::time::Instant;
use file_reader::parse_data::*;  // Import read_file function
use checker::checker::*;
use random_meta::random::*;

fn main() {
    // let solution: Vec<u32> = vec![0, 2, 2, 0, 1, 5, 5, 3, 1, 3, 0, 7, 4, 6, 7, 4, 6];
    //let solution: Vec<u32> = vec![0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7];
    
    // let filename = "src/data/Call_7_Vehicle_3.txt"; 
    // let filename = "src/data/Call_18_Vehicle_5.txt";
    // let filename = "src/data/Call_35_Vehicle_7.txt";
    // let filename = "src/data/Call_80_Vehicle_20.txt";
    // let filename = "src/data/Call_130_Vehicle_40.txt";
    let filename = "src/data/Call_300_Vehicle_90.txt";
    
    let mut instance = read_file(filename);

    // Insert compatibility for outsource truck
    instance.compatibility.insert(instance.num_vehicles + 1, (1 .. instance.num_calls + 1).collect());

    let mut outsource_sol:Vec<u32> = vec![];
    for i in 0 .. instance.num_vehicles {
        outsource_sol.push(0);
    }

    for i in 1 .. instance.num_calls + 1 {
        outsource_sol.push(i);
        outsource_sol.push(i);
    }

    let (init_cost, feasible) = check_feasibility_and_get_cost(&instance, &outsource_sol);
    assert!(feasible);
    
    let mut best_cost: u128 = u128::MAX;
    let mut best_solution : Vec<u32> = Vec::new();
    let mut total_time: u128 = 0;
    let mut total_sum: u128 = 0;

    let mut n = 10;


    for i in 1 ..=10 {
        let now = Instant::now(); 
        let (cost, solution) = run_random(&instance);
        let time_taken = Instant::now().duration_since(now).as_millis();
        total_time += time_taken;
        
        // if no feasible solution found
        if cost == u128::MAX {
            n -= 1;
            continue;
        }

        total_sum += cost;
        
        if cost < best_cost {
            best_cost = cost;
            best_solution = solution;
        }
    }

    let diff = init_cost as f64 - best_cost as f64;
    let improvement = (diff / init_cost as f64) * 100.0;




    println!("Best cost: {best_cost}");
    println!("Best solution: {best_solution:?}");
    println!("Average time for 10k: {}ms", {total_time / 10});
    if n > 0 { println!("Average cost: {}", total_sum / n); }
    println!("Outsource cost: {init_cost}");
    println!("Improvement: {improvement}%");

}

fn run_random(instance: &Instance) -> (u128, Vec<u32>) {
    let mut best_cost: u128 = u128::MAX;
    let mut best_solution : Vec<u32> = Vec::new();
    let mut solution: Vec<u32>;
    let mut cost: u128;
    let mut feasible: bool;

    for i in 0 .. 10_000 {
        solution = get_random_sol(instance.num_calls, instance.num_vehicles + 1, &instance.compatibility);

        (cost, feasible) = check_feasibility_and_get_cost(&instance, &solution);

        if !feasible { continue; }

        if cost < best_cost {
            // println!("{solution:?}");
            best_cost = cost;
            best_solution = solution;
        }
    }

    return (best_cost, best_solution);
}
