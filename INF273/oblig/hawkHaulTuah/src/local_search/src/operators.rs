use log::{debug, error, info, log_enabled, warn, Level};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, iter::zip, u32, usize};
use std::collections::HashMap;
use checker::checker::*;
use file_reader::parse_data::*;
use log::__private_api::loc;
use rand::distr::weighted::WeightedIndex;
use rand::{prelude::*, random_range, rng};

pub fn one_reinsert_focus_dummy_random_feasible(
    instance: &Instance,
    old_route: &Vec<Vec<u32>>
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
            vehicle_to_idx = get_random_compatible_vehicle(call, &instance, false);
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

pub fn one_reinsert_probability(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
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

    route
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
    let num_vehicle_attempts = instance.num_vehicles;
    let mut best_solution = route.clone();
    let mut best_cost = u128::MAX;

    for _ in 0..num_vehicle_attempts {
        // Select vehicle based on weights
        let vehicle_idx = dist.sample(&mut rng);

        // Skip if this is the outsource vehicle (we'll handle that case separately)

        if vehicle_idx == route.len() - 1 {
            let mut candidate = route.clone();
            candidate[vehicle_idx].insert(0, call);
            candidate[vehicle_idx].insert(0, call);
            // Evaluate full solution cost
            let (total_cost, is_feasible) = check_feasibility_and_get_cost(&instance, &candidate);

            // Update best solution if this is better
            if is_feasible && total_cost < best_cost {
                best_solution = candidate;
                best_cost = total_cost;
            }
            continue;
        }

        // Try to find best insertion positions in this vehicle
        if let Some((pickup_idx, delivery_idx, cost)) =
            find_best_insertion_positions(&instance, &route[vehicle_idx], vehicle_idx, call)
        {
            // Create candidate solution
            let mut candidate = route.clone();
            candidate[vehicle_idx].insert(pickup_idx, call);
            // Adjust delivery index if pickup comes before it
            let adj_delivery_idx = if delivery_idx > pickup_idx {
                delivery_idx + 1
            } else {
                delivery_idx
            };
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

pub fn k_reinsert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let k = 10;

    let mut new_sol = old_route.clone();

    for _ in 0..k {
        new_sol = one_reinsert_greedy_insert(&instance, &new_sol);
    }

    new_sol
}

pub fn actual_k_reinsert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let k = instance.num_calls;

    let mut rng = rand::rng();
    let mut call_idx: usize;
    let mut call: u32;
    let mut route = old_route.clone();
    let mut vehicle_from: usize = route.len() - 1;
    let mut chosen_calls: Vec<u32> = Vec::with_capacity(k as usize);

    for _i in 0..k {
        // Prioritize removing a call from the outsource vehicle with 40% probability
        if !route[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
            call_idx = rng.random_range(0..route[vehicle_from].len());
        } else {
            vehicle_from = get_random_vehicle(&route, true);
            call_idx = rng.random_range(0..route[vehicle_from].len());
        }
        call = remove_call_from_vehicle(call_idx, vehicle_from, &mut route);
        chosen_calls.push(call);
    }

    let weights = get_slack_probability(&instance, route.clone(), true);

    // Create weighted distribution for vehicle selection
    for call in chosen_calls {
        let mut weights_clone = weights.clone();

        // Filter out incompatible vehicles
        for i in 0..route.len() - 1 {
            if !instance.compatibility[&((i + 1) as u32)].contains(&call) {
                weights_clone[i] = 0.0;
            }
        }

        let mut vehicle_idx;

        // Try to use weighted selection for compatible vehicles
        match WeightedIndex::new(&weights_clone) {
            Ok(dist) => {
                // Select vehicle based on weights
                vehicle_idx = dist.sample(&mut rng);

                // If it's the outsource vehicle, just add the call
                if vehicle_idx == route.len() - 1 {
                    route[vehicle_idx].push(call);
                    route[vehicle_idx].push(call);
                    continue;
                }

                // Try to find best insertion positions in this vehicle
                if let Some((pickup_idx, delivery_idx, _)) =
                    find_best_insertion_positions(&instance, &route[vehicle_idx], vehicle_idx, call)
                {
                    // Create candidate solution
                    route[vehicle_idx].insert(pickup_idx, call);
                    // Adjust delivery index if pickup comes before it
                    let adj_delivery_idx = if delivery_idx > pickup_idx {
                        delivery_idx + 1
                    } else {
                        delivery_idx
                    };
                    route[vehicle_idx].insert(adj_delivery_idx, call);
                    continue;
                }
            }
            Err(_) => {}
        };

        // If we get here, either:
        // 1. We couldn't create a weighted distribution, or
        // 2. We couldn't find a feasible insertion in the selected vehicle
        // So we'll use the outsource vehicle as a fallback
        let outsource_idx = route.len() - 1;
        route[outsource_idx].push(call);
        route[outsource_idx].push(call);
    }

    return route;
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
    let mut vehicle_indices: Vec<usize> = (0..old_route.len() - 1).collect(); // Skip outsource vehicle
    vehicle_indices.shuffle(&mut rng);
    let vehicle_sample = &vehicle_indices[0..std::cmp::min(30, vehicle_indices.len())];

    // For each sampled vehicle, find valid subroutes
    for &vehicle_idx in vehicle_sample {
        let vehicle = &old_route[vehicle_idx];

        // Skip empty vehicles
        if vehicle.len() < 4 {
            // Need at least 2 calls (4 positions) for a meaningful subroute
            continue;
        }

        // Find valid subroutes more efficiently
        let max_subroute_length = std::cmp::min(90, vehicle.len());

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
                        let original_vehicle_cost =
                            check_feasibility_one_vehicle(instance, &vehicle, vehicle_idx).0;

                        // Calculate cost if we insert this call optimally elsewhere
                        sandbox_solution = insert_best_position(instance, &sandbox_solution, call);
                    }

                    let new_cost = check_feasibility_and_get_cost(instance, &sandbox_solution).0;
                    let delta_cost = original_cost as i128 - new_cost as i128;

                    // If this looks promising, add to candidates
                    if delta_cost > 0 {
                        // Store improvement potential along with the subroute details
                        subroutes.push((
                            vehicle_idx,
                            start,
                            end,
                            delta_cost as u128,
                            subroute_calls.clone(),
                        ));
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

            if let Some((pickup_idx, delivery_idx, cost)) =
                find_best_insertion_positions(instance, &new_solution[v_idx], v_idx, call)
            {
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
    let non_empty_vehicles: Vec<usize> = (0..old_route.len() - 1)
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
        if !instance.compatibility[&((v2_idx + 1) as u32)].contains(&call1)
            || !instance.compatibility[&((v1_idx + 1) as u32)].contains(&call2)
        {
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
        if let Some((p1_idx, d1_idx, _)) =
            find_best_insertion_positions(instance, &new_route[v2_idx], v2_idx, call1)
        {
            // Insert call1 in vehicle2
            new_route[v2_idx].insert(p1_idx, call1);
            let adj_d1_idx = if d1_idx > p1_idx { d1_idx + 1 } else { d1_idx };
            new_route[v2_idx].insert(adj_d1_idx, call1);

            if let Some((p2_idx, d2_idx, _)) =
                find_best_insertion_positions(instance, &new_route[v1_idx], v1_idx, call2)
            {
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

pub fn reorder_random_subroute_excact(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut calls: Vec<u32> = Vec::new();
    let mut rng = rng();
    let mut new_route = old_route.clone();

    let non_empty_indexes: Vec<usize> = (0..old_route.len() - 1)
        .filter(|idx| !old_route[*idx].is_empty() &&
            old_route[*idx].len() < 10).collect();

    if non_empty_indexes.is_empty() {
        return old_route.clone();
    }

    // Choose random vehicle that is not empty.
    let rand_idx = rng.random_range(0 .. non_empty_indexes.len());
    let vehicle_idx = non_empty_indexes[rand_idx];
    calls.extend(new_route[vehicle_idx].iter());

    // Branch on calls to insert or not.
    let new_vehicle_route = insert_calls_recursive(&instance, Vec::new(), vehicle_idx, calls);
    new_route[vehicle_idx] = new_vehicle_route;

    new_route
}

fn insert_calls_recursive(instance: &Instance, route: Vec<u32>, vehicle_id: usize, calls: Vec<u32>) -> Vec<u32> {
    if calls.is_empty() {
        return route;
    }

    let mut results: Vec<Vec<u32>> = (0..calls.len()).into_par_iter()
        .map(|idx| {
            let call = calls[idx];
            let mut local_clone = route.clone();
            let mut local_calls = calls.clone();
            local_calls.retain(|x| *x != call);

            let (_, (i1, i2)) = get_best_insert(&instance, &route, call, vehicle_id);
            local_clone.insert(i1, call);
            local_clone.insert(i2, call);

            local_clone = insert_calls_recursive(&instance, local_clone, vehicle_id, local_calls);
            local_clone
        }).collect();

    /*
    for (i, call) in calls.iter().enumerate() {
        let mut local_clone = route.clone();
        let mut local_calls = calls.clone();
        local_calls.remove(call);

        let (_, (i1, i2)) = get_best_insert(&instance, &route, *call, vehicle_id);

        local_clone.insert(i1, *call);
        local_clone.insert(i2, *call);

        local_clone = insert_calls_recursive(&instance, local_clone, vehicle_id, local_calls);

        results.push(local_clone)
    }*/

    let mut min_cost = u128::MAX;
    let mut min_route = results[0].clone();

    for route in results {
        let cost = check_feasibility_one_vehicle(&instance, &route, vehicle_id).0;
        if cost < min_cost {
            min_cost = cost;
            min_route = route.clone();
        }
    }

    min_route.clone()
}

/*
========================================================
------------------- DESTROY OPERATORS ------------------
========================================================
*/

pub fn one_reinsert_removal(old_route: &Vec<Vec<u32>>, k: u32) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let mut calls = Vec::with_capacity(k as usize);
    let mut route = old_route.clone();
    let mut vehicle_from: usize = route.len() - 1;

    for _i in 0..k {
        // Prioritize removing a call from the outsource vehicle with 40% probability
        if !route[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
            let call_idx = rng.random_range(0..route[vehicle_from].len());
            calls.push(remove_call_from_vehicle(call_idx, vehicle_from, &mut route));
        } else {
            vehicle_from = get_random_vehicle(&route, true);
            let call_idx = rng.random_range(0..route[vehicle_from].len());
            calls.push(remove_call_from_vehicle(call_idx, vehicle_from, &mut route));
        }
    }

    (route, calls)
}

pub fn random_removal(old_route: &Vec<Vec<u32>>, k: u32) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let mut removed_calls = Vec::new();
    let mut new_route = old_route.clone();

    for _i in 0..k {
        let vehicle_idx = get_random_vehicle(&new_route, true);
        let call_idx = rng.random_range(0..new_route[vehicle_idx].len());
        let call = remove_call_from_vehicle(call_idx, vehicle_idx, &mut new_route);
        removed_calls.push(call);
    }

    (new_route, removed_calls)
}

pub fn random_removal_xs(instance: &Instance, old_route: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let k_range = (1..10);
    let k = instance.num_calls.min(rng.random_range(k_range));

    random_removal(old_route, k as u32)
}

pub fn random_removal_s(instance: &Instance, old_route: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let k_range = (10 .. 20);
    let mut k = rng.random_range(k_range);

    if k > instance.num_calls {
        k = 1;
    }

    random_removal(old_route, k as u32)
}

pub fn random_removal_m(instance: &Instance, old_route: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let k_range = (20 .. 50);
    let mut k = rng.random_range(k_range);

    if k > instance.num_calls {
        k = 1;
    }

    random_removal(old_route, k as u32)
}

pub fn random_removal_l(instance: &Instance, old_route: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let k_range = (50 .. 100);
    let mut k = rng.random_range(k_range);

    if k > instance.num_calls {
        k = 1;
    }

    random_removal(old_route, k as u32)
}

pub fn random_removal_dyn(instance: &Instance, old_route: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let k_range = (instance.num_calls as f64 * 0.2 .. instance.num_calls as f64 * 0.6);
    let k = rng.random_range(k_range);

    random_removal(old_route, k as u32)
}

/// Shaw removal - removes related calls based on travel distances and time windows
///
/// This implementation first selects a random call, then removes calls that are
/// spatially and temporally related to the selected call.
pub fn shaw_removal(instance: &Instance, old_route: &Vec<Vec<u32>>, k: u32) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let mut removed_calls = Vec::new();
    let mut new_route = old_route.clone();
    
    // Skip if there are no calls
    if old_route.iter().all(|v| v.is_empty()) {
        return (new_route, removed_calls);
    }
    
    // Get a random first call to remove (seed call)
    let vehicle_idx = get_random_vehicle(&new_route, true);
    if new_route[vehicle_idx].is_empty() {
        return random_removal(old_route, k); // Fallback
    }
    
    let call_idx = rng.random_range(0..new_route[vehicle_idx].len());
    let seed_call = new_route[vehicle_idx][call_idx];
    let call = remove_call_from_vehicle(call_idx, vehicle_idx, &mut new_route);
    removed_calls.push(call);
    
    // Get seed call data
    let seed_call_data = &instance.calls[(seed_call - 1) as usize];
    
    // Collect all remaining calls in the solution
    let mut all_calls = Vec::new();
    for (v_idx, vehicle) in new_route.iter().enumerate() {
        if v_idx == new_route.len() - 1 {
            continue; // Skip outsource vehicle
        }
        
        let vehicle_calls: HashSet<u32> = vehicle.iter().cloned().collect();
        for call_id in vehicle_calls {
            all_calls.push((call_id, v_idx));
        }
    }
    
    // If no more calls left, return early
    if all_calls.is_empty() {
        return (new_route, removed_calls);
    }
    
    // Score all remaining calls by relatedness to the seed call
    let mut relatedness_scores = Vec::new();
    
    for (call_id, vehicle_idx) in all_calls {
        let call_data = &instance.calls[(call_id - 1) as usize];
        let vehicle_id = (vehicle_idx + 1) as u32; // Vehicle indices start at 1 in instance
        
        // Calculate distance score using actual travel costs from instance
        // Get minimum travel cost between the seed call and current call's nodes
        let travel_o1_o2 = instance.travels[&(vehicle_id, seed_call_data.origin, call_data.origin)].cost;
        let travel_o1_d2 = instance.travels[&(vehicle_id, seed_call_data.origin, call_data.destination)].cost;
        let travel_d1_o2 = instance.travels[&(vehicle_id, seed_call_data.destination, call_data.origin)].cost;
        let travel_d1_d2 = instance.travels[&(vehicle_id, seed_call_data.destination, call_data.destination)].cost;
        
        // Get the minimum cost among all combinations
        let min_cost = travel_o1_o2.min(travel_o1_d2).min(travel_d1_o2).min(travel_d1_d2);
        
        // Normalize travel cost (lower is better)
        // We'll use a simple normalization based on a reasonable maximum distance
        let max_expected_cost = 1000_u128;
        let distance_score = min_cost as f64 / max_expected_cost as f64;
        
        // Calculate time window overlap for pickup and delivery
        let pickup_overlap = calculate_time_window_overlap(
            seed_call_data.pickup_start, seed_call_data.pickup_end,
            call_data.pickup_start, call_data.pickup_end
        );
        
        let delivery_overlap = calculate_time_window_overlap(
            seed_call_data.delivery_start, seed_call_data.delivery_end,
            call_data.delivery_start, call_data.delivery_end
        );
        
        // Higher overlap is better, so we invert (lower score is better)
        let time_score = 1.0 - ((pickup_overlap + delivery_overlap) / 2.0);
        
        // Also consider size similarity
        let size_ratio = if seed_call_data.size >= call_data.size {
            call_data.size as f64 / seed_call_data.size as f64
        } else {
            seed_call_data.size as f64 / call_data.size as f64
        };
        
        // Combine scores (lower is more related)
        // We weight distance more heavily than time windows
        let relatedness_score = 0.6 * distance_score + 0.3 * time_score + 0.1 * (1.0 - size_ratio);
        
        relatedness_scores.push((call_id, vehicle_idx, relatedness_score));
    }
    
    if relatedness_scores.is_empty() {
        return (new_route, removed_calls);
    }
    
    // Transform relatedness scores into weights for the WeightedIndex
    // For WeightedIndex, higher weights = higher probability
    // But our scores are reversed (lower score = more related)
    // So we need to invert the scores
    
    // Find the max score to help with inversion
    let max_score = relatedness_scores.iter()
        .map(|(_, _, score)| *score)
        .fold(0.0, |a: f64, b: f64| a.max(b));
        
    // Create a copy of relatedness_scores to work with
    let mut working_scores = relatedness_scores.clone();
    
    // Number of calls to select
    let num_to_select = std::cmp::min(k as usize - 1, working_scores.len());
    
    for _ in 0..num_to_select {
        if working_scores.is_empty() {
            break;
        }
        
        // Recalculate weights each time (since we remove elements)
        let weights: Vec<f64> = working_scores.iter()
            .map(|(_, _, score)| (max_score - score + 0.01))
            .collect();
            
        // Create a new weighted distribution
        match WeightedIndex::new(&weights) {
            Ok(dist) => {
                // Select index using weighted distribution
                let idx = dist.sample(&mut rng);
                
                // Get the call info
                let (call_id, v_idx, _) = working_scores[idx];
                
                // Remove call from working set
                working_scores.remove(idx);
                
                // Find and remove this call from the solution
                if let Some(c_idx) = new_route[v_idx].iter().position(|&c| c == call_id) {
                    let call = remove_call_from_vehicle(c_idx, v_idx, &mut new_route);
                    removed_calls.push(call);
                }
            },
            Err(_) => {
                // Fallback to random selection if weighting fails
                let idx = rng.random_range(0..working_scores.len());
                let (call_id, v_idx, _) = working_scores.remove(idx);
                
                if let Some(c_idx) = new_route[v_idx].iter().position(|&c| c == call_id) {
                    let call = remove_call_from_vehicle(c_idx, v_idx, &mut new_route);
                    removed_calls.push(call);
                }
            }
        }
    }
    
    (new_route, removed_calls)
}

/// Helper function to calculate time window overlap between two time windows
/// Returns a value between 0.0 (no overlap) and 1.0 (complete overlap)
fn calculate_time_window_overlap(start1: u128, end1: u128, start2: u128, end2: u128) -> f64 {
    let latest_start = std::cmp::max(start1, start2);
    let earliest_end = std::cmp::min(end1, end2);
    
    if latest_start <= earliest_end {
        // There is an overlap
        let overlap_length = earliest_end - latest_start;
        let window1_length = end1 - start1;
        let window2_length = end2 - start2;
        
        // Normalize by the average window length
        let avg_window_length = (window1_length + window2_length) / 2;
        if avg_window_length > 0 {
            return overlap_length as f64 / avg_window_length as f64;
        }
    }
    
    0.0 // No overlap
}

pub fn worst_removal(instance: &Instance, old_route: &Vec<Vec<u32>>, k: u32) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut removed_calls = Vec::new();
    let mut new_route = old_route.clone();

    for _i in 0..k {
        let (vehicle_idx, call_idx) = find_most_costly_call(instance, &new_route);
        let call = remove_call_from_vehicle(call_idx, vehicle_idx, &mut new_route);
        removed_calls.push(call);
    }

    (new_route, removed_calls)
}

pub fn route_removal(_instance: &Instance, old_route: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rng();
    let mut removed_calls = HashSet::new();

    let non_empty_vehicles: Vec<usize> = (0..old_route.len())
        .filter(|&idx| !old_route[idx].is_empty())
        .collect();

    let mut new_route = old_route.clone();
    let mut vehicle_idx = rng.random_range(0..non_empty_vehicles.len());
    for c in &new_route[vehicle_idx] {
        removed_calls.insert(*c);
    }
    new_route[vehicle_idx] = Vec::new();

    (new_route, removed_calls.into_iter().collect::<Vec<u32>>())
}

/*
========================================================
------------------- REPAIR OPERATORS -------------------
========================================================
*/

fn greedy_insertion(instance: &Instance, old_route: &Vec<Vec<u32>>, calls_to_insert: Vec<u32>) -> Vec<Vec<u32>> {
    // For small number of calls, just use the sequential approach
    if calls_to_insert.len() <= 3 {
        let mut new_route = old_route.clone();
        for call in calls_to_insert {
            new_route = insert_best_position(&instance, &new_route, call);
        }
        return new_route;
    }
    
    // For larger sets of calls, try multiple insertion orders in parallel
    use rayon::prelude::*;
    use rand::seq::SliceRandom;
    let mut rng = rng();
    
    // Generate multiple random permutations of the calls
    let num_permutations = 8.min(calls_to_insert.len());
    let mut permutations: Vec<Vec<u32>> = Vec::with_capacity(num_permutations);
    
    // First permutation is the original order
    permutations.push(calls_to_insert.clone());
    
    // Generate additional random permutations
    for _ in 1..num_permutations {
        let mut permutation = calls_to_insert.clone();
        permutation.shuffle(&mut rng);
        permutations.push(permutation);
    }
    
    // Process each permutation in parallel
    let results: Vec<(Vec<Vec<u32>>, u128)> = permutations.par_iter()
        .map(|perm| {
            let mut route = old_route.clone();
            for &call in perm {
                route = insert_best_position(&instance, &route, call);
            }
            let cost = check_feasibility_and_get_cost(&instance, &route).0;
            (route, cost)
        })
        .collect();
    
    // Find the best solution
    if let Some((best_route, _)) = results.into_iter()
        .min_by_key(|(_, cost)| *cost) {
        return best_route;
    }
    
    // Fallback to the original solution if something went wrong
    old_route.clone()
}

fn one_reinsert_insertion(instance: &Instance, old_route: &Vec<Vec<u32>>, calls_to_insert: Vec<u32>) -> Vec<Vec<u32>> {
    // Calculate vehicle selection weights based on slack capacity
    let mut route = old_route.clone();
    let mut rng = rng();
    for call in calls_to_insert {
        let mut weights = get_slack_probability(&instance, route.clone(), true);

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
                let outsource_idx = route.len() - 1;
                route[outsource_idx].push(call);
                route[outsource_idx].push(call);
                continue
            }
        };

        // Sample multiple vehicles with probability based on slack
        // Attempt to find good insertion positions in each vehicle
        let num_vehicle_attempts = instance.num_vehicles;
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
            if let Some((pickup_idx, delivery_idx, cost)) =
                find_best_insertion_positions(&instance, &route[vehicle_idx], vehicle_idx, call)
            {
                // Create candidate solution
                let mut candidate = route.clone();
                candidate[vehicle_idx].insert(pickup_idx, call);
                // Adjust delivery index if pickup comes before it
                let adj_delivery_idx = if delivery_idx > pickup_idx {
                    delivery_idx + 1
                } else {
                    delivery_idx
                };
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
            route = best_solution;
            continue
        }

        // Otherwise, use the outsource vehicle as fallback
        let outsource_idx = route.len() - 1;
        route[outsource_idx].push(call);
        route[outsource_idx].push(call);
    }

    return route;
}

/// K-regret insertion heuristic
///
/// Inserts calls one by one, choosing the call with the highest "regret" value at each step.
/// The regret value is the difference between the best insertion cost and the kth best insertion cost.
/// The intuition is that calls with high regret should be inserted first, since they would suffer more
/// from being inserted later when fewer options remain.
///
/// # Parameters
/// * `instance` - Problem instance data
/// * `old_route` - Current route to improve
/// * `calls_to_insert` - Vector of calls that need to be inserted
/// * `k` - The k parameter that determines how many alternative insertions to consider for regret
///
/// # Returns
/// * Updated solution with all calls inserted
pub fn k_regret_insertion(instance: &Instance, old_route: &Vec<Vec<u32>>, calls_to_insert: Vec<u32>, k: usize) -> Vec<Vec<u32>> {
    let mut new_route = old_route.clone();
    let mut remaining_calls = calls_to_insert.clone();

    // Continue until all calls are inserted
    while !remaining_calls.is_empty() {
        // Calculate regret values for all remaining calls
        let mut regret_values = calculate_all_regret_values(instance, &new_route, &remaining_calls, k);
        
        // Find the call with maximum regret (or with feasible insertions)
        let best_call_opt = remaining_calls.iter()
            .filter(|&call| regret_values.contains_key(call))
            .max_by_key(|&call| regret_values[call]);

        if let Some(&best_call) = best_call_opt {
            // Insert the call at its best position
            new_route = insert_best_position(instance, &new_route, best_call);

            // Remove the call from remaining calls
            remaining_calls.retain(|&c| c != best_call);
        } else {
            // No feasible insertions - send remaining calls to outsource
            let outsource_idx = new_route.len() - 1;
            for call in remaining_calls {
                new_route[outsource_idx].push(call);
                new_route[outsource_idx].push(call);
            }
            break;
        }
    }

    new_route
}

fn first_random_feasible_insertion(instance: &Instance, old_route: &Vec<Vec<u32>>, calls_to_insert: Vec<u32>) -> Vec<Vec<u32>> {
    let mut rng = rng();
    let mut result_route = old_route.clone();
    let mut calls_to_insert = calls_to_insert.clone();
    calls_to_insert.shuffle(&mut rng);

    let chance_for_outsource = 1.0 / instance.num_calls as f64;

    for call in calls_to_insert {
        let test_route = result_route.clone();

        if  rand::random::<f64>() < chance_for_outsource {
            result_route[old_route.len() - 1].push(call);
            result_route[old_route.len() - 1].push(call);
            continue;
        }

        result_route = find_first_feasible_insert(&instance, &result_route, call);
    }

    result_route
}


/*
========================================================
------------- DESTROY AND REPAIR OPERATORS -------------
========================================================
*/

pub fn random_xs_greedy(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (new_route, calls) = random_removal_xs(&instance, &old_route);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn random_s_greedy(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (new_route, calls) = random_removal_s(&instance, &old_route);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn random_m_greedy(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (new_route, calls) = random_removal_m(&instance, &old_route);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn random_l_greedy(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (new_route, calls) = random_removal_l(&instance, &old_route);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn random_dyn_greedy(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (new_route, calls) = random_removal_dyn(&instance, &old_route);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn test_all_calls_reinsert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut best_cost = check_feasibility_and_get_cost(&instance, &old_route).0;
    let mut best_solution = old_route.clone();

    for vehicle_idx in 0..old_route.len() {
        for call_idx in 0..old_route[vehicle_idx].len() {
            let mut candidate = old_route.clone();
            let call = remove_call_from_vehicle(call_idx, vehicle_idx, &mut candidate);
            candidate = greedy_insertion(&instance, &candidate, vec![call]);
            let candidate_cost = check_feasibility_and_get_cost(&instance, &candidate).0;
            if candidate_cost < best_cost {
                best_cost = candidate_cost;
                best_solution = candidate;
            }
        }
    }

    best_solution
}

pub fn k_reinsert_real(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let k = rand::random_range((instance.num_calls as f64 * 0.4)..instance.num_calls as f64 * 0.8) as u32;
    let (new_route, calls) = one_reinsert_removal(&old_route, k as u32);

    one_reinsert_insertion(&instance, &new_route, calls)
}

pub fn random_removal_greedy_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let k = rand::random_range((instance.num_calls as f64 * 0.4)..instance.num_calls as f64 * 0.8) as u32;
    let (new_route, calls) = random_removal(&old_route, k as u32);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn worst_removal_greedy_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let k = rand::random_range((instance.num_calls as f64 * 0.4)..instance.num_calls as f64 * 0.8) as u32;

    let (new_route, calls) = worst_removal(&instance, &old_route, k as u32);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn route_removal_greedy_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (new_route, calls) = route_removal(&instance, &old_route);

    greedy_insertion(&instance, &new_route, calls)
}

/// Shaw removal with greedy insertion repair
///
/// This operator removes related calls (based on distance and time windows)
/// and reinserts them using greedy insertion
pub fn shaw_removal_greedy_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let k = rand::random_range((instance.num_calls as f64 * 0.4)..instance.num_calls as f64 * 0.8) as u32;
    let (new_route, calls) = shaw_removal(&instance, &old_route, k as u32);

    greedy_insertion(&instance, &new_route, calls)
}

pub fn random_removal_k_regret_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rng = rng();
    let choices = (3..((instance.num_calls / 5) + 3) as usize).step_by(10).collect::<Vec<_>>();
    let rand_idx = rng.random_range(0..choices.len());
    let k = choices[rand_idx];
    let (new_route, calls) = random_removal(&old_route, k as u32);

    k_regret_insertion(&instance, &new_route, calls, 4)
}

pub fn worst_removal_k_regret_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rng = rng();
    let choices = (3..=((instance.num_calls / 7) + 3) as usize).step_by(10).collect::<Vec<_>>();
    let rand_idx = rng.random_range(0..choices.len());
    let k = choices[rand_idx];
    let (new_route, calls) = worst_removal(&instance, &old_route, k as u32);

    k_regret_insertion(&instance, &new_route, calls, 4)
}

pub fn route_removal_k_regret_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (new_route, calls) = route_removal(&instance, &old_route);

    k_regret_insertion(&instance, &new_route, calls, 4)
}

/// Shaw removal with k-regret insertion repair
/// 
/// This operator removes related calls (based on distance and time windows)
/// and reinserts them using the k-regret heuristic
pub fn shaw_removal_k_regret_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rng = rng();
    let choices = (3..((instance.num_calls / 5) + 3) as usize).step_by(10).collect::<Vec<_>>();
    let rand_idx = rng.random_range(0..choices.len());
    let k = choices[rand_idx];
    let (new_route, calls) = shaw_removal(&instance, &old_route, k as u32);
    
    k_regret_insertion(&instance, &new_route, calls, 4)
}


pub fn random_removal_greedy_insert_10_times(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut resulting_solution = old_route.clone();
    for _i in 0..10{
        resulting_solution = random_removal_greedy_insert(&instance, &resulting_solution);
    }

    resulting_solution
}

pub fn shaw_removal_greedy_insert_10_times(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut resulting_solution = old_route.clone();

    for _i in 0..10{
        resulting_solution = shaw_removal_greedy_insert(&instance, &resulting_solution);
    }

    resulting_solution
}

/// Destroy and repair operator that removes random calls and reinserts them using fast feasible insertion
/// 
/// This operator randomly selects a number of calls to remove, then reinserts them using
/// the first feasible insertion heuristic, which quickly finds any compatible position.
///
/// # Parameters
/// * `instance` - Problem instance data
/// * `old_route` - Current solution
///
/// # Returns
/// * Updated solution with calls reinserted
pub fn random_removal_first_feasible_insert(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // Dynamically determine number of calls to remove - make it proportional to problem size
    // Also add some randomization to the number for exploration
    let mut k = 3.max(instance.num_vehicles);
    
    let (new_route, calls) = random_removal(&old_route, k as u32);

    first_random_feasible_insertion(&instance, &new_route, calls)
}


fn find_most_costly_call(instance: &Instance, route: &Vec<Vec<u32>>) -> (usize, usize) {
    let mut most_costly_vehicle_idx = 0;
    let mut most_costly_call_idx = 0;
    let mut highest_delta = 0;

    for vehicle_idx in 0..route.len() - 1 {
        let vehicle = &instance.vehicles[vehicle_idx];
        for call_idx in 0..route[vehicle_idx].len() {
            let call = route[vehicle_idx][call_idx];
            let actual_call = &instance.calls[(call - 1) as usize];

            // Get current node for call
            let current_node = if route[vehicle_idx][0..call_idx].contains(&call) {
                actual_call.destination
            } else {
                actual_call.origin
            };

            // Get node of previous call
            let prev_node = if call_idx == 0 {
                vehicle.home_node
            } else {
                let prev_call = route[vehicle_idx][call_idx - 1];
                let actual_call = &instance.calls[(prev_call - 1) as usize];
                if route[vehicle_idx][0..call_idx-1].contains(&prev_call) {
                    actual_call.destination
                } else {
                    actual_call.origin
                }
            };

            // Get node of next call
            let next_node = if call_idx == route[vehicle_idx].len() - 1 {
                vehicle.home_node
            } else {
                let next_call = route[vehicle_idx][call_idx + 1];
                let actual_call = &instance.calls[(next_call - 1) as usize];
                if route[vehicle_idx][0..call_idx+1].contains(&next_call) {
                    actual_call.destination
                } else {
                    actual_call.origin
                }
            };

            // Calculate delta cost with and without traveling to current call
            let travel_prev_call = instance.travels[&(vehicle.index, prev_node, current_node)].cost;
            let travel_call_next = instance.travels[&(vehicle.index, current_node, next_node)].cost;
            let travel_prev_next = instance.travels[&(vehicle.index, prev_node, next_node)].cost;

            let delta = travel_prev_call + travel_call_next - travel_prev_next;


            if delta > highest_delta {
                highest_delta = delta;
                most_costly_call_idx = call_idx;
                most_costly_vehicle_idx = vehicle_idx;
            }
        }
    }

    for (call_idx, call) in route[route.len() - 1].iter().enumerate() {
        let actual_call = &instance.calls[(call - 1) as usize];
        if actual_call.cost_outsource > highest_delta {
            most_costly_call_idx = call_idx;
            highest_delta = actual_call.cost_outsource;
            most_costly_vehicle_idx = route.len() - 1;
        }
    }

    (most_costly_vehicle_idx, most_costly_call_idx)
}

/// Find the first feasible insertion for a call with fully randomized positions
///
/// This function tries to insert a call into the solution, checking vehicle-compatible
/// insertions in a random order and with random insertion positions.
/// 
/// # Parameters
/// * `instance` - Problem instance data
/// * `route` - Current solution
/// * `call` - The call to insert
///
/// # Returns
/// * Updated solution with the call inserted
fn find_first_feasible_insert(instance: &Instance, route: &Vec<Vec<u32>>, call: u32) -> Vec<Vec<u32>> {
    let mut rng = rng();
    
    // Get all compatible vehicles (can include empty ones)
    let mut compatible_vehicles: Vec<usize> = (0..route.len() - 1)
        .filter(|vehicle_idx| instance.compatibility[&(*vehicle_idx as u32 + 1)].contains(&call))
        .collect();
    
    // Shuffle the vehicles to check them in random order
    compatible_vehicles.shuffle(&mut rng);
    
    let mut resulting_route = route.clone();
    
    // Try each vehicle in random order
    for vehicle_idx in compatible_vehicles {
        let vehicle = &route[vehicle_idx];
        let vehicle_len = vehicle.len();
        
        // Generate all possible pickup-delivery pairs
        let mut insertion_positions = Vec::new();
        for pickup in 0..=vehicle_len {
            for delivery in pickup..=vehicle_len {
                insertion_positions.push((pickup, delivery));
            }
        }
        
        // Shuffle the insertion positions to try them in random order
        insertion_positions.shuffle(&mut rng);
        
        // Try each pickup-delivery pair in random order
        for (pickup, delivery) in insertion_positions {
            let (_, feasible) = check_insertion_feasibility(
                &instance, 
                vehicle, 
                vehicle_idx, 
                call, 
                pickup, 
                delivery
            );
            
            if feasible {
                resulting_route[vehicle_idx].insert(pickup, call);
                // Adjust delivery index if pickup comes before it
                let adjusted_delivery = if delivery > pickup { delivery + 1 } else { delivery };
                resulting_route[vehicle_idx].insert(adjusted_delivery, call);
                return resulting_route;
            }
        }
    }
    
    // If no feasible insertion found, use outsource vehicle
    let outsource_idx = route.len() - 1;
    
    // Choose random positions in the outsource vehicle
    let outsource_len = resulting_route[outsource_idx].len();
    let pos1 = rng.random_range(0..=outsource_len);
    let pos2 = rng.random_range(0..=outsource_len + 1);
    
    resulting_route[outsource_idx].insert(pos1, call);
    resulting_route[outsource_idx].insert(pos2, call);
    
    resulting_route
}

/// Calculate regret values for all calls that need to be inserted
///
/// # Parameters
/// * `instance` - Problem instance data
/// * `routes` - Current route configuration
/// * `calls` - Vector of calls that need to be inserted
/// * `k` - The k parameter that determines how many alternative insertions to consider
///
/// # Returns
/// * HashMap mapping each call to its regret value
fn calculate_all_regret_values(
    instance: &Instance,
    routes: &Vec<Vec<u32>>,
    calls: &Vec<u32>,
    k: usize
) -> HashMap<u32, i128> {
    let mut regret_values = HashMap::new();

    for &call in calls {
        // Get all insertion costs for this call
        let insertion_costs = get_insertion_costs(instance, routes, call);

        if !insertion_costs.is_empty() {
            // Calculate regret value
            let regret = calculate_regret(&insertion_costs, k);
            regret_values.insert(call, regret);
        }
    }

    regret_values
}

/// Gets the insertion costs for a call across all compatible vehicles
///
/// # Parameters
/// * `instance` - Problem instance data
/// * `routes` - Current routes
/// * `call` - The call to calculate insertion costs for
///
/// # Returns
/// * Vector of insertion costs, sorted from lowest to highest
fn get_insertion_costs(
    instance: &Instance,
    routes: &Vec<Vec<u32>>,
    call: u32
) -> Vec<i128> {
    let mut costs = Vec::new();

    // Check each vehicle (except outsource vehicle)
    for vehicle_idx in 0..routes.len() - 1 {
        // Skip incompatible vehicles
        if !instance.compatibility[&(vehicle_idx as u32 + 1)].contains(&call) {
            continue;
        }

        let route = &routes[vehicle_idx];
        let (cost, _) = get_best_insert(instance, route, call, vehicle_idx);

        // Only add if a feasible insertion was found
        if cost < i128::MAX {
            costs.push(cost);
        }
    }

    // Sort costs in ascending order (lowest cost first)
    costs.sort();

    costs
}

/// Calculate the regret value based on insertion costs
///
/// The regret value is the difference between the best insertion cost and the kth best insertion cost.
/// Higher regret means this call should be inserted early, as its cost increases significantly
/// if it cannot be inserted in its best position.
///
/// # Parameters
/// * `costs` - Vector of sorted insertion costs
/// * `k` - Number of positions to consider for regret
///
/// # Returns
/// * Regret value as i128
fn calculate_regret(costs: &Vec<i128>, k: usize) -> i128 {
    if costs.is_empty() {
        return i128::MIN; // No feasible insertions
    }

    if costs.len() == 1 {
        // Only one feasible insertion location
        // Return negative cost to prioritize calls with low insertion cost when only one option
        return -costs[0];
    }

    // Limit k to the number of available costs
    let actual_k = std::cmp::min(k, costs.len());
    
    // Best (lowest) insertion cost
    let best_cost = costs[0];
    
    // Calculate regret as sum of differences between best cost and k-1 other costs
    let mut regret = 0;
    for i in 1..actual_k {
        regret += costs[i] - best_cost;
    }

    // Higher regret value means higher priority
    regret
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
    let length = if include_outsource {
        route.len()
    } else {
        route.len() - 1
    };

    // Find all non-empty vehicles
    let non_empty_vehicles: Vec<usize> =
        (0..length).filter(|&idx| !route[idx].is_empty()).collect();

    // If no vehicles have calls, return the first regular vehicle
    // This shouldn't happen in practice, but provides a safety fallback
    if non_empty_vehicles.is_empty() {
        return 0; // Return first vehicle as a fallback
    }

    // Randomly select from non-empty vehicles
    return non_empty_vehicles[rng.random_range(0..non_empty_vehicles.len())];
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
    use rayon::prelude::*;
    
    // Calculate slack for all vehicles in parallel
    let weights: Vec<f64> = (0..routes.len() - 1)
        .into_par_iter()
        .map(|i| calculate_vehicle_slack(instance, &routes[i], i) as f64)
        .collect();
    
    // Create a mutable copy to modify
    let mut weights_mut = weights;
    
    // Find the maximum weight
    let max_weight = weights_mut
        .iter()
        .copied()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(1.0);
    
    // Handle outsource vehicle
    if include_outsource {
        weights_mut.push(max_weight / weights_mut.len().max(1) as f64);
    } else {
        weights_mut.push(0.0);
    }
    
    // Set empty routes to maximum weight
    for (i, route) in routes[0..routes.len() - 1].iter().enumerate() {
        if route.is_empty() {
            weights_mut[i] = max_weight;
        }
    }
    
    // Handle edge case where all weights are zero
    if weights_mut.iter().sum::<f64>() == 0.0 {
        weights_mut = vec![1.0; routes.len() - 1];
        weights_mut.push(0.0);
    }
    
    weights_mut
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
        let i1: usize;
        let i2: usize;
        let vehicle_idx: usize;
        let costs: Vec<i128> = results.clone().iter().map(|(cost, _, _, _)| (-cost)).collect();


        match WeightedIndex::new(&costs) {
            Ok(weighted_index) => {
                let idx = weighted_index.sample(&mut rng());
                (_, vehicle_idx, i1, i2) = results[idx].clone();
            },
            Err(_) => {
                (_, vehicle_idx, i1, i2) = results.iter().min_by_key(|(cost, _, _, _)| cost).unwrap().clone();
            }
        }

        let mut new_route = routes.clone();
        let mut vehicle: Vec<u32> = new_route[vehicle_idx].clone();
        vehicle.insert(i1, call);
        vehicle.insert(i2, call);
        new_route[vehicle_idx] = vehicle;

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
        let mut costs: Vec<&u128> = Vec::with_capacity(feasible_insertions.len());
        feasible_insertions.iter().for_each(|(pickup_idx, delivery_idx, delta_cost)| {costs.push(delta_cost)});

        let mut rng = rand::rng();

        /*
        // Try weighted index to choose insertion
        match WeightedIndex::new(costs) {
            Ok(weighted_index) => {
                return Some(feasible_insertions[weighted_index.sample(&mut rng)])
            },
            Err(_) => {                return None
            }
        }*/

        // Return a random insertion from the top 3 best positions (if we have that many)
        let top_n = std::cmp::min(3, feasible_insertions.len());

        let selected_idx = rng.random_range(0..top_n);
        let (pickup_idx, delivery_idx, cost) = feasible_insertions[selected_idx];

        return Some((pickup_idx, delivery_idx, cost));
    }

    None
}

