use checker::checker::*;
use file_reader::parse_data::*;
use rand::prelude::*;


pub enum Operator {
    OneReinsert,
    TwoExchange,
    ThreeExchange
}

pub fn run_local_search(init_solution: &Vec<Vec<u32>>, operator: Operator, instance: &Instance) -> (Vec<Vec<u32>>, u128){
    let mut best_solution = init_solution.clone();
    let (mut best_cost, a) = check_feasibility_and_get_cost(&instance, &best_solution);
    
    for _i in 0 .. 10_000 {
        let mut new_solution = best_solution.clone();
        one_reinsert(&mut new_solution);

        let (cost, feasible) = check_feasibility_and_get_cost(instance, &new_solution);
        if feasible && cost < best_cost {
            best_solution = new_solution;
            best_cost = cost;
        }
    }

    return (best_solution, best_cost);
}

pub fn one_reinsert(solution: &mut Vec<Vec<u32>>) {
    let mut rng = rand::thread_rng();

    let mut vehicle_from = rng.gen_range(0..solution.len());
    while solution[vehicle_from].is_empty() {
        vehicle_from = rng.gen_range(0..solution.len());
    }

    let call_idx = rng.gen_range(0..solution[vehicle_from].len());
    let call = solution[vehicle_from].remove(call_idx);
    
    if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
        solution[vehicle_from].remove(index);
    } else { panic!("There were not two calls in vehicle") }

    let vehicle_to = rng.gen_range(0..solution.len());
    let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
    solution[vehicle_to].insert(insert_idx, call);
    let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
    solution[vehicle_to].insert(insert_idx, call);
    
}