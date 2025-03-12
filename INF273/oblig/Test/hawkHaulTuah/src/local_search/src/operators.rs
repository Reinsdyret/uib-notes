use log::{debug, error, info, log_enabled, warn, Level};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, iter::zip, u32, usize};

use checker::checker::*;
use file_reader::parse_data::*;
use rand::distr::weighted::WeightedIndex;
use rand::{prelude::*, rng};

pub fn one_reinsert_focus_dummy_random_feasible(
    old_route: &Vec<Vec<u32>>,
    instance: &Instance,
) -> Vec<Vec<u32>> {
    let mut rng = rand::rng();
    let call: u32;
    let mut route = old_route.clone();
    let mut vehicle_from: usize = route.len() - 1;
    let include_outsource: bool;
    // Choosing call, prioritizing outsource vehicle but still random call.
    // Random vehicle if outsource vehicle is empty
    if !route[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
        let call_idx = rng.random_range(0..route[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut route);
        include_outsource = false;
        // println!("Chose call {call}");
    } else {
        vehicle_from = get_random_vehicle(&route, true);
        let call_idx = rng.random_range(0..route[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut route);
        include_outsource = true;
    }

    let mut vehicle_to: Vec<u32>;
    let mut vehicle_to_idx: u32;
    let mut insert_idx1: usize;
    let mut insert_idx2: usize;
    let mut i: usize = 0;

    if !route[vehicle_from].is_empty() && rand::random::<f64>() < 0.1 {
        vehicle_to_idx = (route.len() - 1) as u32;

        insert_idx1 = rng.random_range(0..=route[route.len() - 1].len());
        insert_idx2 = rng.random_range(0..=route[route.len() - 1].len());
    } else {
        loop {
            // Find random but compatible vehicle
            vehicle_to_idx = get_random_compatible_vehicle(call, &instance, include_outsource);
            vehicle_to = route[vehicle_to_idx as usize].clone();

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

    let vehicle_to = &mut route[vehicle_to_idx as usize];
    vehicle_to.insert(insert_idx1, call);
    vehicle_to.insert(insert_idx2, call);

    return route;
}

pub fn one_reinsert_probability(old_route: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    let mut rng = rand::rng();
    let call: u32;
    let mut route = old_route.clone();
    let mut vehicle_from: usize = route.len() - 1;
    let include_outsource: bool;
    // Choosing call, prioritizing outsource vehicle but still random call.
    // Random vehicle if outsource vehicle is empty
    if !route[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
        // Value here changes improvement alot
        let call_idx = rng.random_range(0..route[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut route);
        include_outsource = false;
        // println!("Chose call {call}");
    } else {
        vehicle_from = get_random_vehicle(&route, true);
        let call_idx = rng.random_range(0..route[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut route);
        include_outsource = true;
    }

    let mut vehicle_to: Vec<u32>;
    let mut vehicle_to_idx: usize;
    let mut insert_idx1: usize;
    let mut insert_idx2: usize;
    let mut i: usize = 0;

    let mut weights = get_slack_probability(&instance, route.clone(), include_outsource);
    for i in 0..route.len() - 1 {
        if !instance.compatibility[&((i + 1) as u32)].contains(&call) {
            weights[i] = 0.0;
        }
    }
    let dist = WeightedIndex::new(&weights).unwrap();

    loop {
        // Find random but compatible vehicle
        vehicle_to_idx = dist.sample(&mut rng);
        vehicle_to = route[vehicle_to_idx].clone();

        // Get a distribution of feasible inserts for that vehicle

        // Insert calls in random position
        insert_idx1 = rng.random_range(0..=vehicle_to.len());
        insert_idx2 = rng.random_range(0..=vehicle_to.len());
        vehicle_to.insert(insert_idx1, call);
        vehicle_to.insert(insert_idx2, call);

        if vehicle_to_idx as usize == route.len() - 1
            || check_feasibility_one_vehicle(&instance, &vehicle_to, vehicle_to_idx as usize).1
        {
            break;
        }
        if i >= 100 {
            break;
        }
        i += 1;
    }

    let vehicle_to = &mut route[vehicle_to_idx as usize];
    vehicle_to.insert(insert_idx1, call);
    vehicle_to.insert(insert_idx2, call);

    return route;
}

pub fn one_reinsert_greedy_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rng = rand::rng();
    let call: u32;
    let mut route = old_route.clone();
    let mut vehicle_from: usize = route.len() - 1;
    let include_outsource: bool;
    
    // Prioritize removing a call from the outsource vehicle with 40% probability
    if !route[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
        let call_idx = rng.random_range(0..route[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut route);
        include_outsource = false;
    } else {
        vehicle_from = get_random_vehicle(&route, true);
        let call_idx = rng.random_range(0..route[vehicle_from].len());
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut route);
        include_outsource = true;
    }

    // Calculate vehicle selection weights based on slack capacity
    let mut weights = get_slack_probability(&instance, route.clone(), include_outsource);
    
    // Filter out incompatible vehicles
    for i in 0..route.len() - 1 {
        if !instance.compatibility[&((i + 1) as u32)].contains(&call) {
            weights[i] = 0.0;
        }
    }
    
    // Create weighted distribution for vehicle selection
    let dist = match WeightedIndex::new(&weights) {
        Ok(d) => d,
        Err(_) => {
            // Fallback if all weights are zero - just use the outsource vehicle
            let mut new_route = route.clone();
            let outsource_idx = new_route.len() - 1;
            new_route[outsource_idx].push(call);
            new_route[outsource_idx].push(call);
            return new_route;
        }
    };
    
    // Sample multiple vehicles with probability based on slack
    // Attempt to find good insertion positions in each vehicle
    let num_vehicle_attempts = 5;
    let mut best_solution = route.clone();
    let mut best_cost = u128::MAX;
    
    for _ in 0..num_vehicle_attempts {
        // Select vehicle based on weights
        let vehicle_idx = dist.sample(&mut rng);
        
        // Skip if this is the outsource vehicle (we'll handle that case separately)
        if vehicle_idx == route.len() - 1 {
            continue;
        }
        
        // Try to find best insertion positions in this vehicle
        if let Some((pickup_idx, delivery_idx, cost)) = find_best_insertion_positions(
            &instance, 
            &route[vehicle_idx], 
            vehicle_idx, 
            call
        ) {
            // Create candidate solution
            let mut candidate = route.clone();
            candidate[vehicle_idx].insert(pickup_idx, call);
            // Adjust delivery index if pickup comes before it
            let adj_delivery_idx = if delivery_idx > pickup_idx { delivery_idx + 1 } else { delivery_idx };
            candidate[vehicle_idx].insert(adj_delivery_idx, call);
            
            // Evaluate full solution cost
            let (total_cost, is_feasible) = check_feasibility_and_get_cost(&instance, &candidate);
            
            // Update best solution if this is better
            if is_feasible && total_cost < best_cost {
                best_solution = candidate;
                best_cost = total_cost;
            }
        }
    }
    
    // If we found a feasible insertion with better cost, return it
    if best_cost < u128::MAX {
        return best_solution;
    }
    
    // Otherwise, use the outsource vehicle as fallback
    let mut outsource_solution = route.clone();
    let outsource_idx = outsource_solution.len() - 1;
    outsource_solution[outsource_idx].push(call);
    outsource_solution[outsource_idx].push(call);
    
    return outsource_solution;
}

pub fn try_k_reinserts(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rng = rng();
    let k = 50;

    let mut costs: Vec<u128> = Vec::new();
    let mut reinserts: Vec<Vec<Vec<u32>>> = Vec::new();

    for _i in 0..k {
        let mut vehicle_from_idx = rng.random_range(0..old_route.len());

        while old_route[vehicle_from_idx].len() == 0 {
            vehicle_from_idx = rng.random_range(0..old_route.len());
        }

        let call_idx = rng.random_range(0..old_route[vehicle_from_idx].len());

        let mut sol_copy = old_route.clone();
        let call = remove_call_from_vehicle(call_idx, vehicle_from_idx, &mut sol_copy);

        sol_copy = insert_best_position(&instance, &sol_copy, call);

        let cost = check_feasibility_and_get_cost(&instance, &sol_copy).0;

        costs.push(cost);
        reinserts.push(sol_copy);
    }

    match WeightedIndex::new(&costs) {
        Ok(dist) => {
            // Select an index based on weights
            let selected_idx = dist.sample(&mut rng);
            return reinserts[selected_idx].clone();
        }
        Err(_) => {
            return one_reinsert_probability(&old_route, &instance);
        }
    }
}

pub fn remove_from_vehicle_travel_long(
    instance: &Instance,
    old_route: &Vec<Vec<u32>>,
) -> Vec<Vec<u32>> {
    todo!();
}

pub fn reinsert_sub_route(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // Take a valid subroute from one vehicle and reinsert its calls in better positions
    let mut subroutes: Vec<(usize, usize, usize, u128, Vec<u32>)> = Vec::new();
    let mut rng = rand::rng();
    
    // Only sample a subset of vehicles to improve performance
    let mut vehicle_indices: Vec<usize> = (0..old_route.len()-1).collect(); // Skip outsource vehicle
    vehicle_indices.shuffle(&mut rng);
    let vehicle_sample = &vehicle_indices[0..std::cmp::min(30, vehicle_indices.len())];
    
    // For each sampled vehicle, find valid subroutes
    for &vehicle_idx in vehicle_sample {
        let vehicle = &old_route[vehicle_idx];
        
        // Skip empty vehicles
        if vehicle.len() < 4 {  // Need at least 2 calls (4 positions) for a meaningful subroute
            continue;
        }

        // Find valid subroutes more efficiently
        let max_subroute_length = std::cmp::min(30, vehicle.len());
        
        for subroute_len in 2..=max_subroute_length {
            for start in 0..=vehicle.len() - subroute_len {
                let end = start + subroute_len - 1;
                let subroute = &vehicle[start..=end];
                
                if is_valid_subroute(subroute) {
                    // Create a temporary solution with this subroute removed
                    let mut new_vehicle = vehicle.clone();
                    remove_subroute(&mut new_vehicle, start, end);
                    
                    // Create a new solution and estimate improvement
                    let mut new_solution = old_route.clone();
                    new_solution[vehicle_idx] = new_vehicle;
                    
                    // Extract unique calls in the subroute
                    let unique_calls: HashSet<u32> = HashSet::from_iter(subroute.iter().cloned());
                    let subroute_calls: Vec<u32> = unique_calls.into_iter().collect();
                    
                    // Calculate the delta cost
                    let original_cost = check_feasibility_and_get_cost(instance, old_route).0;
                    
                    // Create a sandbox solution to test improvement potential
                    let mut sandbox_solution = new_solution.clone();
                    
                    // First pass: quickly estimate potential improvement
                    for &call in &subroute_calls {
                        // Calculate cost of call in current position
                        let original_vehicle_cost = check_feasibility_one_vehicle(instance, &vehicle, vehicle_idx).0;
                        
                        // Calculate cost if we insert this call optimally elsewhere
                        sandbox_solution = insert_best_position(instance, &sandbox_solution, call);
                    }
                    
                    let new_cost = check_feasibility_and_get_cost(instance, &sandbox_solution).0;
                    let delta_cost = original_cost as i128 - new_cost as i128;
                    
                    // If this looks promising, add to candidates
                    if delta_cost > 0 {
                        // Store improvement potential along with the subroute details
                        subroutes.push((vehicle_idx, start, end, delta_cost as u128, subroute_calls.clone()));
                    }
                }
            }
        }
    }
    
    // If we found no promising subroutes, return the original solution
    if subroutes.is_empty() {
        return old_route.clone();
    }
    
    // Sort subroutes by delta_cost (higher first) and take top candidates
    subroutes.sort_by_key(|(_, _, _, cost, _)| std::cmp::Reverse(*cost));
    
    // Take the top 3 subroutes or fewer if we don't have 3
    let top_n = std::cmp::min(3, subroutes.len());
    let selected_idx = rng.random_range(0..top_n);
    let (vehicle_idx, start, end, _, subroute_calls) = &subroutes[selected_idx];
    
    // Remove the selected subroute
    let mut new_solution = old_route.clone();
    let mut vehicle = new_solution[*vehicle_idx].clone();
    remove_subroute(&mut vehicle, *start, *end);
    new_solution[*vehicle_idx] = vehicle;
    
    // Insert each call in the best position using our new greedy insertion approach
    for &call in subroute_calls {
        // Try to find best destinations for this call
        let mut best_vehicle_idx = new_solution.len() - 1; // Default to outsource
        let mut best_pickup_idx = 0;
        let mut best_delivery_idx = 1;
        let mut best_insertion_cost = u128::MAX;
        
        // Check multiple potential vehicles for insertion
        for v_idx in 0..new_solution.len() - 1 {
            if !instance.compatibility[&((v_idx + 1) as u32)].contains(&call) {
                continue;
            }
            
            if let Some((pickup_idx, delivery_idx, cost)) = find_best_insertion_positions(
                instance,
                &new_solution[v_idx],
                v_idx,
                call
            ) {
                if cost < best_insertion_cost {
                    best_insertion_cost = cost;
                    best_vehicle_idx = v_idx;
                    best_pickup_idx = pickup_idx;
                    best_delivery_idx = delivery_idx;
                }
            }
        }
        
        // Insert at the best position found
        if best_vehicle_idx == new_solution.len() - 1 {
            // Outsource
            new_solution[best_vehicle_idx].push(call);
            new_solution[best_vehicle_idx].push(call);
        } else {
            // Regular vehicle
            new_solution[best_vehicle_idx].insert(best_pickup_idx, call);
            // Adjust the delivery index if pickup came before it
            let adj_delivery_idx = if best_delivery_idx > best_pickup_idx { 
                best_delivery_idx + 1 
            } else { 
                best_delivery_idx 
            };
            new_solution[best_vehicle_idx].insert(adj_delivery_idx, call);
        }
    }
    
    new_solution
}


pub fn two_call_swap(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // An enhanced version of two_call_swap that tries more combinations
    let mut rng = rand::rng();
    let mut best_route = old_route.clone();
    let (best_cost, _) = check_feasibility_and_get_cost(instance, &best_route);
    
    // Get list of non-empty vehicles (excluding outsource)
    let non_empty_vehicles: Vec<usize> = (0..old_route.len()-1)
        .filter(|&idx| !old_route[idx].is_empty())
        .collect();
    
    // Need at least 2 non-empty vehicles
    if non_empty_vehicles.len() < 2 {
        return best_route;
    }
    
    // Try multiple combinations for better results
    let num_attempts = 30;
    
    for _ in 0..num_attempts {
        // Select two different vehicles with probability based on route length
        // Vehicles with more calls have higher probability of selection
        let mut vehicle_weights: Vec<f64> = non_empty_vehicles
            .iter()
            .map(|&idx| old_route[idx].len() as f64)
            .collect();
        
        if vehicle_weights.is_empty() || vehicle_weights.iter().sum::<f64>() == 0.0 {
            return best_route;
        }
        
        let dist = match WeightedIndex::new(&vehicle_weights) {
            Ok(d) => d,
            Err(_) => return best_route,
        };
        
        let v1_idx_pos = dist.sample(&mut rng);
        let v1_idx = non_empty_vehicles[v1_idx_pos];
        
        // Temporarily remove the selected vehicle for second selection
        let mut remaining_vehicles = non_empty_vehicles.clone();
        remaining_vehicles.remove(v1_idx_pos);
        let v2_idx_pos = rng.random_range(0..remaining_vehicles.len());
        let v2_idx = remaining_vehicles[v2_idx_pos];
        
        // Get a set of candidate calls from each vehicle
        let unique_calls_v1: HashSet<u32> = HashSet::from_iter(old_route[v1_idx].iter().cloned());
        let unique_calls_v2: HashSet<u32> = HashSet::from_iter(old_route[v2_idx].iter().cloned());
        
        // Convert to vectors for random access
        let calls_v1: Vec<u32> = unique_calls_v1.into_iter().collect();
        let calls_v2: Vec<u32> = unique_calls_v2.into_iter().collect();
        
        if calls_v1.is_empty() || calls_v2.is_empty() {
            continue;
        }
        
        // Select one random call from each vehicle
        let call1 = calls_v1[rng.random_range(0..calls_v1.len())];
        let call2 = calls_v2[rng.random_range(0..calls_v2.len())];
        
        // Check vehicle compatibility for both calls
        if !instance.compatibility[&((v2_idx + 1) as u32)].contains(&call1) || 
           !instance.compatibility[&((v1_idx + 1) as u32)].contains(&call2) {
            continue;
        }
        
        // Create a new solution with the calls removed
        let mut new_route = old_route.clone();
        
        // Find and remove call1 from vehicle1
        let call1_pos1 = new_route[v1_idx].iter().position(|&x| x == call1).unwrap();
        new_route[v1_idx].remove(call1_pos1);
        let call1_pos2 = new_route[v1_idx].iter().position(|&x| x == call1).unwrap();
        new_route[v1_idx].remove(call1_pos2);
        
        // Find and remove call2 from vehicle2
        let call2_pos1 = new_route[v2_idx].iter().position(|&x| x == call2).unwrap();
        new_route[v2_idx].remove(call2_pos1);
        let call2_pos2 = new_route[v2_idx].iter().position(|&x| x == call2).unwrap();
        new_route[v2_idx].remove(call2_pos2);
        
        // Find best insertion positions for each call in the other vehicle
        if let Some((p1_idx, d1_idx, _)) = find_best_insertion_positions(instance, &new_route[v2_idx], v2_idx, call1) {
            // Insert call1 in vehicle2
            new_route[v2_idx].insert(p1_idx, call1);
            let adj_d1_idx = if d1_idx > p1_idx { d1_idx + 1 } else { d1_idx };
            new_route[v2_idx].insert(adj_d1_idx, call1);
            
            if let Some((p2_idx, d2_idx, _)) = find_best_insertion_positions(instance, &new_route[v1_idx], v1_idx, call2) {
                // Insert call2 in vehicle1
                new_route[v1_idx].insert(p2_idx, call2);
                let adj_d2_idx = if d2_idx > p2_idx { d2_idx + 1 } else { d2_idx };
                new_route[v1_idx].insert(adj_d2_idx, call2);
                
                // Check if new solution is feasible and better
                let (new_cost, is_feasible) = check_feasibility_and_get_cost(instance, &new_route);
                
                if is_feasible && new_cost < best_cost {
                    best_route = new_route;
                    break; // Exit early if we found an improvement
                }
            }
        }
    }
    
    best_route
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

fn get_random_vehicle(route: &Vec<Vec<u32>>, include_outsource: bool) -> usize {
    let mut rng = rand::rng();
    let mut vehicle = rng.random_range(0..route.len() - 1);
    if include_outsource {
        vehicle = rng.random_range(0..route.len());
    }
    let mut i = 0;
    while route[vehicle].is_empty() {
        vehicle = rng.random_range(0..route.len() - 1);
        if i == 100 {
            vehicle = route.len() - 1;
            if route[vehicle].is_empty() {
                info!("COULD NOT FIND ANY RANDOM VEHICLE WITH ANY CALLS");
            }
            return vehicle;
        }
        i += 1;
    }

    return vehicle;
}

fn remove_call_from_vehicle(
    call_idx: usize,
    vehicle_from: usize,
    route: &mut Vec<Vec<u32>>,
) -> u32 {
    let call = route[vehicle_from].remove(call_idx);
    if let Some(index) = route[vehicle_from].iter().position(|&x| x == call) {
        route[vehicle_from].remove(index);
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

fn insert_best_position(instance: &Instance, routes: &Vec<Vec<u32>>, call: u32) -> Vec<Vec<u32>> {
    // Run parallell to check best insert on all vehicles and calculate delta costs

    let results: Vec<(i128, usize, usize, usize)> = (0..routes.len() - 1)
        .into_par_iter()
        .filter_map(|vehicle_idx| {
            let route = &routes[vehicle_idx];
            if !instance.compatibility[&(vehicle_idx as u32 + 1)].contains(&call) {
                return None;
            }
            let (cost, (i1, i2)) = get_best_insert(&instance, &route, call, vehicle_idx);
            if cost >= i128::MAX {
                return None;
            }
            return Some((cost, vehicle_idx, i1, i2));
        })
        .collect();

    if !results.is_empty() {
        let (_, vehicle_idx, i1, i2) = results.iter().min_by_key(|(cost, _, _, _)| cost).unwrap();

        let mut new_route = routes.clone();
        let mut vehicle: Vec<u32> = new_route[*vehicle_idx].clone();
        vehicle.insert(*i1, call);
        vehicle.insert(*i2, call);
        new_route[*vehicle_idx] = vehicle;

        return new_route;
    }

    let mut outsource_solution = routes.clone();
    let outsource_idx = outsource_solution.len() - 1;
    outsource_solution[outsource_idx].push(call);
    outsource_solution[outsource_idx].push(call);

    return outsource_solution;
}

fn get_delta_cost_with_insert(
    instance: &Instance,
    route: &Vec<u32>,
    vehicle_idx: usize,
    call: u32,
    pickup_idx: usize,
    delivery_idx: usize,
) -> u128 {
    // Calculate delta cost for insert
    let delta_pickup_cost =
        get_delta_cost_one_insert(&instance, &route, vehicle_idx, call, pickup_idx);

    // Calculate delta cost for pickup
    let delta_delivery_cost =
        get_delta_cost_one_insert(&instance, &route, vehicle_idx, call, delivery_idx);

    return delta_pickup_cost + delta_delivery_cost;
}

fn get_delta_cost_one_insert(
    instance: &Instance,
    route: &Vec<u32>,
    vehicle_idx: usize,
    call: u32,
    index: usize,
) -> u128 {
    let call_before: u32;

    if index == 0 {
        call_before = instance.vehicles[vehicle_idx].home_node;
    } else {
        let call_value = route[index - 1];
        if route[0..index - 1].contains(&call_value) {
            call_before = instance.calls[call_value as usize - 1].destination;
        } else {
            call_before = instance.calls[call_value as usize - 1].origin;
        }
    }

    let call_after_value = route[index];
    let call_after: u32;
    if route[0..index].contains(&call_after_value) {
        call_after = instance.calls[call_after_value as usize - 1].destination;
    } else {
        call_after = instance.calls[call_after_value as usize - 1].origin;
    }
    let vehicle = (vehicle_idx + 1) as u32;

    let actual_call = &instance.calls[call as usize - 1];

    let initial_cost = instance.travels[&(vehicle, call_before, call_after)].cost;

    let insert_cost1 = instance.travels[&(vehicle, call_before, actual_call.origin)].cost;
    let insert_cost2 = instance.travels[&(vehicle, actual_call.destination, call_after)].cost;

    return (insert_cost1 + insert_cost2) - initial_cost;
}

fn get_best_insert(
    instance: &Instance,
    route: &Vec<u32>,
    call: u32,
    vechicle_id: usize,
) -> (i128, (usize, usize)) {
    if route.len() == 0 {
        let (cost, feas) = check_insertion_feasibility(&instance, &route, vechicle_id, call, 0, 1);
        if feas {
            return (0 - cost as i128, (0, 1));
        }
    }
    let mut min_delta_cost = i128::MAX;
    let mut min_delta_pair = (0, 0);

    let baseline_cost = check_feasibility_one_vehicle(&instance, &route, vechicle_id).0;

    for i in 0..=route.len() {
        for j in i + 1..=route.len() {
            let (cost, feasible) =
                check_insertion_feasibility(&instance, &route, vechicle_id, call, i, j);

            if feasible {
                let delta_cost = cost as i128 - baseline_cost as i128;

                if delta_cost < min_delta_cost {
                    min_delta_cost = delta_cost;
                    min_delta_pair = (i, j);
                }
            } else {
                break;
            }
        }
    }

    return (min_delta_cost, min_delta_pair);
}

fn get_call_outsource_costs_inversly(
    instance: &Instance,
    outsource_vehicle: &Vec<u32>,
) -> Vec<f64> {
    let call_set: HashSet<&u32> = HashSet::from_iter(outsource_vehicle.iter());
    let mut costs: Vec<f64> = Vec::new();

    for call_idx in call_set {
        let call: &Call = &instance.calls[(call_idx - 1) as usize];
        costs.push(1.0 / call.cost_outsource as f64);
    }

    return costs;
}

// Check if a subroute contains complete pickup-delivery pairs
fn is_valid_subroute(subroute: &[u32]) -> bool {
    let mut call_count = std::collections::HashMap::new();

    // Count occurrences of each call
    for &call in subroute {
        if call == 0 {
            return false; // Vehicle separator found inside subroute
        }

        *call_count.entry(call).or_insert(0) += 1;
    }

    // Check that each call appears exactly 0 or 2 times
    for &count in call_count.values() {
        if count != 0 && count != 2 {
            return false;
        }
    }

    true
}

fn remove_subroute(route: &mut Vec<u32>, start_idx: usize, end_idx: usize) {
    route.drain(start_idx..=end_idx);
}

/// Find the best positions to insert a call's pickup and delivery in a vehicle route
/// Returns (pickup_index, delivery_index, estimated_cost) if a feasible insertion is found
/// Returns None if no feasible insertion is found
fn find_best_insertion_positions(
    instance: &Instance,
    route: &Vec<u32>,
    vehicle_idx: usize,
    call: u32,
) -> Option<(usize, usize, u128)> {
    if !instance.compatibility[&((vehicle_idx + 1) as u32)].contains(&call) {
        return None;
    }
    
    // Store all feasible insertion positions with their costs
    let mut feasible_insertions = Vec::new();
    let num_positions = route.len() + 1;
    let baseline_cost = check_feasibility_one_vehicle(instance, route, vehicle_idx).0;
    
    // Try all possible combinations of pickup and delivery positions
    for pickup_idx in 0..num_positions {
        // Delivery must come after pickup to maintain invariant
        for delivery_idx in 0..num_positions {
            // Check if this insertion combination is feasible
            let (cost, is_feasible) = check_insertion_feasibility(
                instance,
                route,
                vehicle_idx,
                call,
                pickup_idx,
                delivery_idx,
            );
            
            if is_feasible {
                let delta_cost = if cost > baseline_cost {
                    cost - baseline_cost
                } else {
                    0 // Cost improvement is good!
                };
                
                feasible_insertions.push((pickup_idx, delivery_idx, delta_cost));
            }
        }
    }
    
    // If we found any feasible insertions, return the one with minimum cost
    if !feasible_insertions.is_empty() {
        // Sort by cost (lowest first)
        feasible_insertions.sort_by_key(|(_, _, cost)| *cost);
        
        // Return a random insertion from the top 3 best positions (if we have that many)
        let top_n = std::cmp::min(3, feasible_insertions.len());
        let mut rng = rand::rng();
        let selected_idx = rng.random_range(0..top_n);
        let (pickup_idx, delivery_idx, cost) = feasible_insertions[selected_idx];
        
        return Some((pickup_idx, delivery_idx, cost));
    }
    
    None
}
