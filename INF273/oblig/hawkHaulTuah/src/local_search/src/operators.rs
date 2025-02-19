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

    if !solution[vehicle_from].is_empty()
        && rand::random::<f64>() < 1.0 / (instance.num_vehicles as f64 * 2.0)
    {
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

            if vehicle_to_idx as usize == solution.len() - 1
                || check_feasibility_one_vehicle(&instance, &vehicle_to, vehicle_to_idx as usize).1
            {
                break;
            }
            if i >= 10000 {
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

    let mut weights = get_slack_probability(&instance, solution.clone());
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

fn get_random_compatible_vehicle(call: u32, instance: &Instance, include_outsource: bool) -> u32 {
    let mut rng = rand::rng();
    let max_vehicle = if include_outsource {
        instance.num_vehicles
    } else {
        instance.num_vehicles - 1
    };
    let mut vehicle_to_idx: u32 = rng.random_range(0..max_vehicle) as u32;
    while !instance.compatibility[&(vehicle_to_idx + 1)].contains(&call) {
        vehicle_to_idx = rng.random_range(0..instance.num_vehicles) as u32;
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

fn get_slack_probability(instance: &Instance, routes: Vec<Vec<u32>>) -> Vec<f64> {
    let mut weights: Vec<f64> = Vec::new();

    for (i, route) in routes[0..routes.len() - 1].iter().enumerate() {
        weights.push(calculate_vehicle_slack(&instance, &route, i) as f64);
    }

    weights.push(weights.iter().sum::<f64>() / (weights.len() as f64));

    if weights.iter().sum::<f64>() == 0.0 {
        weights = vec![1.0; routes.len() - 1];
        weights.push(0.0);
    }
    let max_weight = weights
        .clone()
        .into_iter()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();

    for (i, route) in routes[0..routes.len() - 1].iter().enumerate() {
        if route.len() == 0 {
            weights[i] = max_weight;
        }
    }

    return weights;
}

fn calculate_vehicle_slack(instance: &Instance, route: &Vec<u32>, vehicle_idx: usize) -> f64 {
    let vehicle = &instance.vehicles[vehicle_idx];
    let mut total_slack: u128 = 0;
    let mut time: u128 = vehicle.start_time;
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

            total_slack += call.pickup_end - time;

            if time < call.pickup_start {
                time = call.pickup_start;
            }

            time += loading.origin_time;
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
        }
    }

    //println!("{:?}", ((total_slack as f64) / (seen.len() as f64)) as f64);
    if ((total_slack as f64) / (seen.len() as f64)).is_nan() {
        return 0.0;
    }
    return ((total_slack as f64) / (seen.len() as f64)) as f64;
    //return (big1 + big2) as f64;
}
