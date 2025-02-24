use std::{collections::HashSet, iter::zip, u32, usize};

use checker::checker::*;
use file_reader::parse_data::*;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

pub fn one_reinsert_focus_dummy_random_feasible(
    old_solution: &Vec<Vec<u32>>,
    instance: &Instance,
) -> Vec<Vec<u32>> {
    let mut rng = rand::rng();
    let call: u32;
    let mut solution = old_solution.clone();
    let mut vehicle_from: usize = solution.len() - 1;
    let include_outsource: bool;
    // Choosing call, prioritizing outsource vehicle but still random call.
    // Random vehicle if outsource vehicle is empty
    if !solution[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
        // Value here changes improvement alot
        let call_idx = rng.random_range(0..solution[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);
        include_outsource = false;
        // println!("Chose call {call}");
    } else {
        vehicle_from = get_random_vehicle(&solution);
        let call_idx = rng.random_range(0..solution[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);
        include_outsource = true;
    }

    let mut vehicle_to: Vec<u32>;
    let mut vehicle_to_idx: u32;
    let mut insert_idx1: usize;
    let mut insert_idx2: usize;
    let mut i: usize = 0;

    if !solution[vehicle_from].is_empty() && rand::random::<f64>() < 0.1 {
        vehicle_to_idx = (solution.len() - 1) as u32;

        insert_idx1 = rng.random_range(0..=solution[solution.len() - 1].len());
        insert_idx2 = rng.random_range(0..=solution[solution.len() - 1].len());
    } else {
        loop {
            // Find random but compatible vehicle
            vehicle_to_idx = get_random_compatible_vehicle(call, &instance, false);
            vehicle_to = solution[vehicle_to_idx as usize].clone();

            // Insert calls in random position
            insert_idx1 = rng.random_range(0..=vehicle_to.len());
            insert_idx2 = rng.random_range(0..=vehicle_to.len());
            vehicle_to.insert(insert_idx1, call);
            vehicle_to.insert(insert_idx2, call);

            if check_feasibility_one_vehicle(&instance, &vehicle_to, vehicle_to_idx as usize).1 {
                break;
            }
            if i >= 100 {
                break;
            }
            i += 1;
        }
    }

    let vehicle_to = &mut solution[vehicle_to_idx as usize];
    vehicle_to.insert(insert_idx1, call);
    vehicle_to.insert(insert_idx2, call);

    return solution;
}

pub fn one_reinsert_probability(
    old_solution: &Vec<Vec<u32>>,
    instance: &Instance,
) -> Vec<Vec<u32>> {
    let mut rng = rand::rng();
    let call: u32;
    let mut solution = old_solution.clone();
    let mut vehicle_from: usize = solution.len() - 1;
    let include_outsource: bool;
    // Choosing call, prioritizing outsource vehicle but still random call.
    // Random vehicle if outsource vehicle is empty
    if !solution[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
        // Value here changes improvement alot
        let call_idx = rng.random_range(0..solution[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);
        include_outsource = false;
        // println!("Chose call {call}");
    } else {
        vehicle_from = get_random_vehicle(&solution);
        let call_idx = rng.random_range(0..solution[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);
        include_outsource = true;
    }

    let mut vehicle_to: Vec<u32>;
    let mut vehicle_to_idx: usize;
    let mut insert_idx1: usize;
    let mut insert_idx2: usize;
    let mut i: usize = 0;

    let mut weights = get_slack_probability(&instance, solution.clone(), include_outsource);
    for i in 0..solution.len() - 1 {
        if !instance.compatibility[&((i + 1) as u32)].contains(&call) {
            weights[i] = 0.0;
        }
    }
    let dist = WeightedIndex::new(&weights).unwrap();

    loop {
        // Find random but compatible vehicle
        vehicle_to_idx = dist.sample(&mut rng);
        vehicle_to = solution[vehicle_to_idx].clone();

        // Get a distribution of feasible inserts for that vehicle

        // Insert calls in random position
        insert_idx1 = rng.random_range(0..=vehicle_to.len());
        insert_idx2 = rng.random_range(0..=vehicle_to.len());
        vehicle_to.insert(insert_idx1, call);
        vehicle_to.insert(insert_idx2, call);

        if vehicle_to_idx as usize == solution.len() - 1
            || check_feasibility_one_vehicle(&instance, &vehicle_to, vehicle_to_idx as usize).1
        {
            break;
        }
        if i >= 100 {
            break;
        }
        i += 1;
    }

    let vehicle_to = &mut solution[vehicle_to_idx as usize];
    vehicle_to.insert(insert_idx1, call);
    vehicle_to.insert(insert_idx2, call);

    return solution;
}

pub fn rearrange(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    // This sucks balls, can never find a feasible solution
    // Rearrange one random vehicle until the result is feasible or max iteration
    let mut rng = rand::rng();
    let vehicle_idx = rng.random_range(0..old_solution.len() - 1); // Dont include outsource

    let mut vehicle_route = old_solution[vehicle_idx].clone();
    vehicle_route.shuffle(&mut rng);

    let mut i = 1;

    while i <= 2000 && !check_feasibility_one_vehicle(&instance, &vehicle_route, vehicle_idx).1 {
        let vehicle_idx = rng.random_range(0..old_solution.len() - 1); // Dont include outsource

        let mut vehicle_route = old_solution[vehicle_idx].clone();
        vehicle_route.shuffle(&mut rng);
        i += 1;
    }

    if !check_feasibility_one_vehicle(&instance, &vehicle_route, vehicle_idx).1 {
        return one_reinsert_probability(&old_solution, &instance);
    }

    let mut solution = old_solution.clone();
    solution[vehicle_idx] = vehicle_route;

    return solution;
}

pub fn two_opt(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    // Sucks balls
    let mut rng = rand::rng();
    let mut solution = old_solution.clone();

    // Try up to 20 different 2-opt moves
    for _ in 0..2000 {
        let vehicle_idx = rng.random_range(0..solution.len() - 1);
        let route = &mut solution[vehicle_idx];
        if route.len() < 4 {
            continue;
        }
        // Select two random positions in the route
        let pos1 = rng.random_range(0..route.len() - 1);
        let pos2 = rng.random_range(pos1 + 1..route.len());
        if pos1.abs_diff(pos2) < 2 { continue; }
        
        // Create a new route with the segment between pos1 and pos2 reversed
        let mut new_route = route.clone();
        new_route[pos1..=pos2].reverse();
        
        // If valid and feasible, apply the change
        if check_feasibility_one_vehicle(instance, &new_route, vehicle_idx).1 {
            *route = new_route;
            break;
        }
    }

    solution
}

pub fn adjacent_swap(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    // SUCKS SHITE
    let mut rng = rand::rng();
    let mut solution = old_solution.clone();
    
    // Try up to 20 different vehicles
    for _ in 0..20 {
        let vehicle_idx = rng.random_range(0..solution.len() - 1); // Don't include outsource
        let route = &mut solution[vehicle_idx];
        
        if route.len() < 4 { // Need at least 2 calls (4 positions) to swap
            continue;
        }

        // Try up to 10 random positions in the route
        for _ in 0..10 {
            let pos = rng.random_range(0..route.len() - 1);
            let mut new_route = route.clone();
            
            // Swap the calls at positions pos and pos+1
            new_route.swap(pos, pos + 1);
            
            // Check if the new route is feasible and has better cost
            let (new_cost, is_feasible) = check_feasibility_one_vehicle(instance, &new_route, vehicle_idx);
            let (old_cost, _) = check_feasibility_one_vehicle(instance, route, vehicle_idx);
            
            if is_feasible && new_cost < old_cost {
                *route = new_route;
                return solution;
            }
        }
    }

    solution
}

fn get_random_compatible_vehicle(call: u32, instance: &Instance, include_outsource: bool) -> u32 {
    let mut rng = rand::rng();
    let max_vehicle = if include_outsource {
        instance.num_vehicles
    } else {
        instance.num_vehicles - 1
    };
    let mut vehicle_to_idx: u32 = rng.random_range(0..max_vehicle) as u32;
    while !instance.compatibility[&(vehicle_to_idx + 1)].contains(&call) {
        vehicle_to_idx = rng.random_range(0..max_vehicle) as u32;
    }

    return vehicle_to_idx;
}

fn get_random_vehicle(solution: &Vec<Vec<u32>>) -> usize {
    let mut rng = rand::rng();
    let mut vehicle = rng.random_range(0..solution.len());
    while solution[vehicle].is_empty() {
        vehicle = rng.random_range(0..solution.len());
    }

    return vehicle;
}

fn remove_call_from_vehicle(
    call_idx: usize,
    vehicle_from: usize,
    solution: &mut Vec<Vec<u32>>,
) -> u32 {
    let call = solution[vehicle_from].remove(call_idx);
    if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
        solution[vehicle_from].remove(index);
    } else {
        panic!("There were not two calls in vehicle")
    }

    return call;
}

fn get_slack_probability(
    instance: &Instance,
    routes: Vec<Vec<u32>>,
    include_outsource: bool,
) -> Vec<f64> {
    let mut weights: Vec<f64> = Vec::new();

    for (i, route) in routes[0..routes.len() - 1].iter().enumerate() {
        weights.push(calculate_vehicle_slack(&instance, &route, i) as f64);
    }

    let max_weight = weights
        .clone()
        .into_iter()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();

    if include_outsource {
        weights.push(max_weight / weights.len() as f64);
    } else {
        weights.push(0.0)
    }

    for (i, route) in routes[0..routes.len() - 1].iter().enumerate() {
        if route.len() == 0 {
            weights[i] = max_weight;
        }
    }

    if weights.iter().sum::<f64>() == 0.0 {
        weights = vec![1.0; routes.len() - 1];
        weights.push(0.0);
    }

    return weights;
}

fn calculate_vehicle_slack(instance: &Instance, route: &Vec<u32>, vehicle_idx: usize) -> f64 {
    let vehicle = &instance.vehicles[vehicle_idx];
    let mut total_slack: u128 = 0;
    let mut total_capacity: u128 = vehicle.capacity;
    let mut max_slack: u128 = 0;
    let mut max_capacity: u128 = 0;
    let mut time: u128 = vehicle.start_time;
    let mut capacity = vehicle.capacity;
    let mut seen: HashSet<u32> = HashSet::new();
    let mut prev_node = vehicle.home_node;

    let mut big1: u128 = 0;
    let mut big2: u128 = 0;

    for call_idx in route {
        let call = &instance.calls[(call_idx - 1) as usize];
        let loading = &instance.loadings[&(vehicle.index, *call_idx)];

        if !seen.contains(call_idx) {
            seen.insert(*call_idx);
            let travel = &instance.travels[&(vehicle.index, prev_node, call.origin)];
            prev_node = call.origin;

            time += travel.time;

            let slack = call.pickup_end - time;
            if slack > big1 {
                big2 = big1;
                big1 = slack;
            } else if slack > big2 {
                big2 = slack;
            }

            total_slack += slack;

            if time < call.pickup_start {
                time = call.pickup_start;
            }

            time += loading.origin_time;

            // Capacity summation
            capacity -= call.size;
            let capacity_diff = vehicle.capacity - capacity;
            total_capacity += capacity_diff;

            if capacity_diff > max_capacity {
                max_capacity = capacity_diff;
            }
        } else {
            let travel = &instance.travels[&(vehicle.index, prev_node, call.destination)];
            prev_node = call.destination;

            time += travel.time;

            let slack = call.delivery_end - time;
            if slack > big1 {
                big2 = big1;
                big1 = slack;
            } else if slack > big2 {
                big2 = slack;
            }
            total_slack += call.delivery_end - time;

            if time < call.delivery_start {
                time = call.delivery_start;
            }

            time += loading.destination_time;

            // Capacity summation
            capacity += call.size;
            let capacity_diff = vehicle.capacity - capacity;
            total_capacity += capacity_diff;

            if capacity_diff > max_capacity {
                max_capacity = capacity_diff;
            }
        }
    }

    //println!("{:?}", ((total_slack as f64) / (seen.len() as f64)) as f64);
    // if ((total_slack as f64) / (seen.len() as f64)).is_nan() {
    //     return 0.0;
    // }
    // return ((total_slack as f64) / (seen.len() as f64)) as f64;
    //return (big1 + big2) as f64;

    let avg_capacity = (total_capacity as f64) / (route.len() as f64);
    let avg_slack = (total_slack as f64) / (route.len() as f64);

    let norm_capacity = avg_capacity / (max_capacity as f64);
    let norm_slack = avg_slack / (big1 as f64);

    let combined_score = 0.7 * norm_capacity + 0.3 * norm_slack;

    if combined_score.is_normal() {
        return combined_score;
    }
    return 0.0;
}

fn get_all_feasible_inserts(
    instance: &Instance,
    solution: &Vec<u32>,
    call: u32,
    vechicle_id: usize,
) -> (Vec<f64>, Vec<(usize, usize)>) {
    let mut costs: Vec<f64> = Vec::new();
    let mut feasible: Vec<(usize, usize)> = Vec::new();

    for i in 0..solution.len() {
        for j in i..solution.len() {
            let mut test_solution = solution.clone();
            test_solution.insert(i, call);
            test_solution.insert(j, call);

            let (cost, feas) = check_feasibility_one_vehicle(instance, &test_solution, vechicle_id);
            if feas {
                costs.push(cost as f64);
                feasible.push((i, j));
            }
        }
    }

    return (costs, feasible);
}


