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

pub fn remove_from_vehicle_travel_long(
    instance: &Instance,
    old_route: &Vec<Vec<u32>>,
) -> Vec<Vec<u32>> {
    todo!();
}

pub fn reinsert_sub_route(instance: &Instance, old_route: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // Take a valid subroute from one vehicle
    // How to choose:
    //   Delta cost with and without that route
    //   Cant be that many valid subroutes
    //
    // Reinsert calls one by one in another vehicle
    let mut subroutes: Vec<(usize, usize, usize, u128)> = Vec::new();

    let _ = (0..old_route.len()).for_each(|vehicle_idx| {
        let vehicle = old_route[vehicle_idx].clone();
        let mut valid_subroutes = Vec::new();
        for i in 0..vehicle.len() {
            for j in i..vehicle.len() {
                let subroute = &vehicle[i..=j];

                if is_valid_subroute(subroute) {
                    valid_subroutes.push((i, j));
                }
            }
        }

        if valid_subroutes.is_empty() {
            info!("No subroutes found");
        }

        for (start, end) in valid_subroutes {
            let mut new_vehicle = vehicle.clone();

            remove_subroute(&mut new_vehicle, start, end);

            let mut cost_without: u128;
            let mut cost_with: u128;

            // Has to check if vehicle is outsource and calculate new cost
            if vehicle_idx == old_route.len() - 1 {
                let calls: HashSet<&u32> = HashSet::from_iter(old_route[vehicle_idx].iter());
                cost_with = 0;
                cost_without = 0;
                for call in calls {
                    cost_with += instance.calls[*call as usize - 1].cost_outsource;
                }

                let calls_with: HashSet<&u32> = HashSet::from_iter(new_vehicle.iter());
                for call in calls_with {
                    cost_without += instance.calls[*call as usize - 1].cost_outsource;
                }
            } else {
                let calls: HashSet<&u32> = HashSet::from_iter(vehicle[start..=end].iter());
                let mut new_solution = old_route.clone();
                new_solution[vehicle_idx] = new_vehicle.clone();
                for call in calls {
                    insert_best_position(&instance, &mut new_solution, *call);
                }
                cost_without = check_feasibility_and_get_cost(&instance, &new_solution).0;
                cost_with = check_feasibility_and_get_cost(&instance, &old_route).0;
            }

            let delta_cost = cost_with as i128 - cost_without as i128;

            if delta_cost > 0 {
                subroutes.push((vehicle_idx, start, end, delta_cost as u128));
            }
        }
    });

    let (vehicle_idx, start, end, delta_cost) = if !subroutes.is_empty() {
        // Create weights based on delta costs
        // Need to ensure all weights are positive
        let mut weights: Vec<u128> = subroutes.iter().map(|(_, _, _, cost)| *cost).collect();

        // If there are any negative or zero delta costs, we need to adjust
        let min_cost = weights.iter().min().unwrap_or(&1);
        if *min_cost <= 0 {
            // Add an offset to make all weights positive
            weights = weights.iter().map(|&cost| (cost) as u128).collect();
        }

        // Create the weighted distribution
        match WeightedIndex::new(&weights) {
            Ok(dist) => {
                // Select an index based on weights
                let mut rng = rng();
                let selected_idx = dist.sample(&mut rng);
                subroutes[selected_idx].clone()
            }
            Err(_) => {
                // Fallback to max selection if weighted index creation fails
                *subroutes.iter().max_by_key(|(_, _, _, cost)| cost).unwrap()
            }
        }
    } else {
        // Handle empty subroutes case
        return old_route.clone();
    };

    // Insert the calls
    let mut new_solution = old_route.clone();
    let mut vehicle: Vec<u32> = new_solution[vehicle_idx].clone();
    let subroute = &vehicle.clone()[start..=end];
    let calls: HashSet<&u32> = HashSet::from_iter(subroute.iter());

    remove_subroute(&mut vehicle, start, end);

    new_solution[vehicle_idx] = vehicle;

    for call in calls {
        new_solution = insert_best_position(&instance, &new_solution, *call);
    }

    return new_solution;
}

pub fn rearrange(old_route: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    // This sucks balls, can never find a feasible route
    // Rearrange one random vehicle until the result is feasible or max iteration
    let mut rng = rand::rng();
    let vehicle_idx = rng.random_range(0..old_route.len() - 1); // Dont include outsource

    let mut vehicle_route = old_route[vehicle_idx].clone();
    vehicle_route.shuffle(&mut rng);

    let mut i = 1;

    while i <= 2000 && !check_feasibility_one_vehicle(&instance, &vehicle_route, vehicle_idx).1 {
        let vehicle_idx = rng.random_range(0..old_route.len() - 1); // Dont include outsource

        let mut vehicle_route = old_route[vehicle_idx].clone();
        vehicle_route.shuffle(&mut rng);
        i += 1;
    }

    if !check_feasibility_one_vehicle(&instance, &vehicle_route, vehicle_idx).1 {
        return one_reinsert_probability(&old_route, &instance);
    }

    let mut route = old_route.clone();
    route[vehicle_idx] = vehicle_route;

    return route;
}

pub fn two_opt(old_route: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    // Sucks balls
    let mut rng = rand::rng();
    let mut route = old_route.clone();

    // Try up to 20 different 2-opt moves
    for _ in 0..2000 {
        let vehicle_idx = rng.random_range(0..route.len() - 1);
        let route = &mut route[vehicle_idx];
        if route.len() < 4 {
            continue;
        }
        // Select two random positions in the route
        let pos1: usize = rng.random_range(0..route.len() - 1);
        let pos2: usize = rng.random_range(pos1 + 1..route.len());
        if pos1.abs_diff(pos2) < 2 {
            continue;
        }

        // Create a new route with the segment between pos1 and pos2 reversed
        let mut new_route = route.clone();
        new_route[pos1..=pos2].reverse();

        // If valid and feasible, apply the change
        if check_feasibility_one_vehicle(instance, &new_route, vehicle_idx).1 {
            *route = new_route;
            break;
        }
    }

    route
}

pub fn adjacent_swap(old_route: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
    // SUCKS SHITE
    let mut rng = rand::rng();
    let mut routes = old_route.clone();

    // Try up to 20 different vehicles
    for _ in 0..20 {
        let vehicle_idx = rng.random_range(0..routes.len() - 1); // Don't include outsource
        let route = &mut routes[vehicle_idx];

        if route.len() < 4 {
            // Need at least 2 calls (4 positions) to swap
            continue;
        }

        // Try up to 10 random positions in the route
        for _ in 0..10 {
            let pos = rng.random_range(0..route.len() - 1);
            let mut new_route = route.clone();

            // Swap the calls at positions pos and pos+1
            new_route.swap(pos, pos + 1);

            // Check if the new route is feasible and has better cost
            let (new_cost, is_feasible) =
                check_feasibility_one_vehicle(instance, &new_route, vehicle_idx);
            let (old_cost, _) = check_feasibility_one_vehicle(instance, route, vehicle_idx);

            if is_feasible && new_cost < old_cost {
                *route = new_route;
                return routes;
            }
        }
    }

    routes
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

    let results: Vec<(u128, usize, usize, usize)> = (0..routes.len() - 1)
        .into_par_iter()
        .map(|vehicle_idx| {
            let route = routes[vehicle_idx].clone();
            if !instance.compatibility[&(vehicle_idx as u32 + 1)].contains(&call) {
                return (u128::MAX, routes.len() - 1, 0, 0);
            }
            let (cost, (i1, i2)) = get_best_insert(&instance, &route, call, vehicle_idx);
            return (cost, vehicle_idx, i1, i2);
        })
        .collect();

    let (_, vehicle_idx, i1, i2) = results.iter().min_by_key(|(cost, _, _, _)| cost).unwrap();

    let mut new_route = routes.clone();
    let mut vehicle: Vec<u32> = new_route[*vehicle_idx].clone();
    vehicle.insert(*i1, call);
    vehicle.insert(*i2, call);
    new_route[*vehicle_idx] = vehicle;

    return new_route;
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
) -> (u128, (usize, usize)) {
    let mut min_delta_cost = u128::MAX;
    let mut min_delta_pair = (0, 0);

    for i in 0..route.len() {
        for j in i..route.len() {
            let cost = get_delta_cost_with_insert(&instance, &route, vechicle_id, call, i, j);

            if cost < min_delta_cost {
                min_delta_cost = cost;
                min_delta_pair = (i, j);
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
