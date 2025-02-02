
use core::time;
use std::u32;
use std::time::Instant;
use file_reader::parse_data::*;  // Import read_file function
use checker::checker::*;
use random_meta::random::*;

fn main() {
    // let solution: Vec<u32> = vec![0, 2, 2, 0, 1, 5, 5, 3, 1, 3, 0, 7, 4, 6, 7, 4, 6];
    //let solution: Vec<u32> = vec![0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7];
    
    let filename = "src/data/Call_7_Vehicle_3.txt";  // Update the path as needed
    
    let mut instance = read_file(filename);

    // Insert compatibility for outsource truck
    instance.compatibility.insert(instance.num_vehicles + 1, (1 .. instance.num_calls + 1).collect());

    let mut outsource_sol:Vec<u32> = vec![0,0,0];
    for i in 1 .. instance.num_calls + 1 {
        outsource_sol.push(i);
        outsource_sol.push(i);
    }

    let (init_cost, feasible) = check_feasibility_and_get_cost(&instance, &outsource_sol);
    assert!(feasible);
    
    let mut best_cost: u32 = u32::MAX;
    let mut best_solution : Vec<u32> = Vec::new();
    let mut best_time: u128 = 0;
    let mut sum_cost:u32 = 0;

    

    for i in 0 .. 10 {
        let now = Instant::now(); 
        let (cost, solution) = run_random(&instance);
        let time_taken = Instant::now().duration_since(now).as_millis();

        sum_cost += cost;
        
        if cost < best_cost {
            best_cost = cost;
            best_solution = solution;
            best_time = time_taken;
        }
    }

    let diff = init_cost - best_cost;
    let improvement: f32 = ((init_cost as f32) / (diff as f32)) * 100f32;



    println!("Best Score: {best_cost}");
    println!("Best solution: {best_solution:?}");
    println!("Time for best solution: {best_time}");
    println!("Average cost: {0}", sum_cost / 10);
    println!("Improvement: {improvement}%");

}

fn run_random(instance: &Instance) -> (u32, Vec<u32>) {
    let mut best_cost: u32 = u32::MAX;
    let mut best_solution : Vec<u32> = Vec::new();
    let mut solution: Vec<u32> = Vec::new();
    let mut cost: u32;
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
