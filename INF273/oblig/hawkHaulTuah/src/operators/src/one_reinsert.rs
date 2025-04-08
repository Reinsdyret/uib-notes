use std::collections::HashSet;
use rand::*;
use rand::distr::weighted::WeightedIndex;
use solution::solution::Solution;
use crate::operator::Operator;

pub struct OneReinsert;

impl Operator for OneReinsert {
    fn name(&self) -> &str {
        "OneReinsert"
    }

    fn apply(&self, solution: &Solution) -> Solution {
        let mut rng = rand::rng();
        let call: u32;
        let mut new_solution = Solution::new(solution.instance, solution.routes.clone());
        let mut vehicle_from: usize = new_solution.routes.len() - 1;
        let include_outsource: bool;

        // Prioritize removing a call from the outsource vehicle with 40% probability
        if !new_solution.routes[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 {
            let call_idx = rng.random_range(0..new_solution.routes[vehicle_from].len());
            call = new_solution.remove_call_from_vehicle(vehicle_from, call_idx);
            include_outsource = false;
        } else {
            if let Some(v) = new_solution.get_random_vehicle() {
                vehicle_from = v;
            } else {
                return new_solution;
            }
            let call_idx = rng.random_range(0..new_solution.routes[vehicle_from].len());
            call = new_solution.remove_call_from_vehicle(vehicle_from, call_idx);
            include_outsource = true;
        }

        // Calculate vehicle selection weights based on slack capacity
        let mut weights = get_slack_probability(&new_solution, include_outsource);

        // Filter out incompatible vehicles
        for i in 0..new_solution.routes.len() - 1 {
            if !new_solution.instance.compatibility[&((i + 1) as u32)].contains(&call) {
                weights[i] = 0.0;
            }
        }

        // Create weighted distribution for vehicle selection
        let mut dist = match WeightedIndex::new(&weights) {
            Ok(d) => d,
            Err(_) => {
                // Fallback if all weights are zero - just use the outsource vehicle
                let outsource_idx = new_solution.routes.len() - 1;
                new_solution.routes[outsource_idx].push(call);
                new_solution.routes[outsource_idx].push(call);
                return new_solution;
            }
        };

        // Sample multiple vehicles with probability based on slack
        // Attempt to find good insertion positions in each vehicle
        let num_vehicle_attempts = new_solution.instance.num_vehicles;
        let mut best_solution = Solution::new(new_solution.instance, new_solution.routes.clone());
        let mut best_cost = u128::MAX;

        for _ in 0..num_vehicle_attempts {
            // Select vehicle based on weights
            let vehicle_idx = dist.sample(&mut rng);

            // Skip if this is the outsource vehicle (we'll handle that case separately)
            if vehicle_idx == best_solution.routes.len() - 1 {
                continue;
            }

            // Try to find best insertion positions in this vehicle
            if let Some((pickup_idx, delivery_idx, cost)) =
                new_solution.find_best_insertion_for_vehicle(vehicle_idx, call)
            {
                // Create candidate solution
                let mut candidate = Solution::new(new_solution.instance, new_solution.routes.clone());
                candidate.routes[vehicle_idx].insert(pickup_idx, call);
                // Adjust delivery index if pickup comes before it
                let adj_delivery_idx = if delivery_idx > pickup_idx {
                    delivery_idx + 1
                } else {
                    delivery_idx
                };
                candidate.routes[vehicle_idx].insert(adj_delivery_idx, call);

                // Evaluate full solution cost
                match candidate.check_feasibility() {
                    Ok(cost) => {
                        if cost < best_cost {
                            best_solution.routes = candidate.routes;
                            best_cost = cost;
                        }
                    },
                    Err(_) => {}
                }
            }
        }

        // If we found a feasible insertion with better cost, return it
        if best_cost < u128::MAX {
            return best_solution;
        }

        // Otherwise, use the outsource vehicle as fallback
        let mut outsource_solution = Solution::new(new_solution.instance, new_solution.routes.clone());
        let outsource_idx = outsource_solution.routes.len() - 1;
        outsource_solution.routes[outsource_idx].push(call);
        outsource_solution.routes[outsource_idx].push(call);

        outsource_solution
    }
}

fn get_slack_probability(
    solution: &Solution,
    include_outsource: bool,
) -> Vec<f64> {
    let mut weights: Vec<f64> = Vec::new();
    let routes = solution.routes.clone();

    for (i, route) in routes[0..routes.len() - 1].iter().enumerate() {
        weights.push(calculate_vehicle_slack(&solution, i) as f64);
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

    weights
}

fn calculate_vehicle_slack(solution: &Solution, vehicle_idx: usize) -> f64 {
    let vehicle = &solution.instance.vehicles[vehicle_idx];
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

    let route = &solution.routes[vehicle_idx];

    for call_idx in route {
        let call = &solution.instance.calls[(call_idx - 1) as usize];
        let loading = &solution.instance.loadings[&(vehicle.index, *call_idx)];

        if !seen.contains(call_idx) {
            seen.insert(*call_idx);
            let travel = &solution.instance.travels[&(vehicle.index, prev_node, call.origin)];
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
            let travel = &solution.instance.travels[&(vehicle.index, prev_node, call.destination)];
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
    0.0
}