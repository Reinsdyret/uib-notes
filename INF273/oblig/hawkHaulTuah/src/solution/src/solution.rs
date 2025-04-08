use std::collections::HashSet;
use rayon::prelude::*;
use instance::instance::*;
use rand::{rng, Rng};

#[derive(Debug)]
pub struct Solution<'a> {
    pub instance: &'a Instance,
    pub routes:  Vec<Vec<u32>>
}

#[derive(Debug, Copy, Clone)]
pub enum Infeasible {
    Incompatible,
    CapacityExceeded,
    PickupExceeded,
    DeliveryExceeded
}

impl<'a> Solution<'a> {
    pub fn new<>(instance: &'a Instance, routes: Vec<Vec<u32>>) -> Self {
        Self { instance, routes }
    }

    pub fn check_feasibility(&self) -> Result<u128, Infeasible> {
        let result = (0..self.routes.len()).into_par_iter().map(|route| {
            self.check_feasibility_one_vehicle(route)}).collect::<Vec<_>>();

        for res in &result {
            if let Err(e) = res {
                return Err(e.clone());
            }
        }

        let cost : u128 = result.iter().map(|res| res.as_ref().unwrap()).sum();

        Ok(cost)
    }

    pub fn check_feasibility_one_vehicle(&self, vehicle_id: usize) -> Result<u128, Infeasible> {
        let calls = &self.instance.calls;
        let travels = &self.instance.travels;
        let loadings = &self.instance.loadings;

        let mut cost: u128 = 0;

        let vehicle: &Vehicle = &self.instance.vehicles[vehicle_id];
        let mut time: u128 = vehicle.start_time;
        let mut capacity: u128 = vehicle.capacity;

        let mut seen: HashSet<u32> = HashSet::new(); // Seen nodes, to know if pickup or delivery
        let mut prev_node: u32 = vehicle.home_node;
        let route: &Vec<u32> = &self.routes[vehicle_id];

        // Normal processing for nodes that are by trucks
        for call_index in route.into_iter() {
            // Calculate running cost.
            // Verify if end time is less than current time then not feasible
            // Verity if capacity is over limit / < 0. Remember to add and remove capacity when loading and unloading.
            // One case for pickup

            if !loadings.contains_key(&(vehicle.index, *call_index)) {
                return Err(Infeasible::Incompatible);
            }

            let call = &calls[(call_index - 1) as usize];
            let loading = &loadings[&(vehicle.index, *call_index)];

            if !seen.contains(call_index) {
                seen.insert(*call_index);

                // Add cost and time to travel to pickup node for call
                let travel = &travels[&(vehicle.index, prev_node, call.origin)];
                prev_node = call.origin;

                cost += travel.cost;
                time += travel.time;

                if call.pickup_end < time {
                    return Err(Infeasible::PickupExceeded);
                }

                // If time < pickup_start -> wait
                if time < call.pickup_start {
                    time = call.pickup_start;
                }

                // Check if space and pickup package
                if capacity < call.size {
                    return Err(Infeasible::CapacityExceeded);
                }
                capacity -= call.size;

                // Add cost and time for loading
                cost += loading.origin_cost;
                time += loading.origin_time;
            } else {
                // Add cost and time to travel to delivery node for call
                let travel = &travels[&(vehicle.index, prev_node, call.destination)];
                prev_node = call.destination;

                cost += travel.cost;
                time += travel.time;

                // One for delivery
                let call = &calls[(call_index - 1) as usize];

                if call.delivery_end < time {
                    return Err(Infeasible::DeliveryExceeded);
                }

                // wait for delivery window
                if time < call.delivery_start {
                    time = call.delivery_start;
                }

                // Deliver package
                capacity += call.size;

                // Add cost and time for loading
                cost += loading.destination_cost;
                time += loading.destination_time;
            }
        }
        Ok(cost)
    }

    pub fn check_insertion_feasibility(
        &self,
        vehicle_id: usize,
        call_to_insert: u32,
        pickup_index: usize,
        delivery_index: usize,
    ) -> Result<u128, Infeasible> {
        let calls = &self.instance.calls;
        let travels = &self.instance.travels;
        let loadings = &self.instance.loadings;
        let vehicle = &self.instance.vehicles[vehicle_id];

        // Check if vehicle is compatible with call
        if !loadings.contains_key(&(vehicle.index, call_to_insert)) {
            return Err(Infeasible::Incompatible);
        }

        // Get call details
        let call = &calls[(call_to_insert - 1) as usize];
        let loading = &loadings[&(vehicle.index, call_to_insert)];

        // Initialize time and capacity for simulation
        let mut time: u128 = vehicle.start_time;
        let mut capacity: u128 = vehicle.capacity;
        let mut cost: u128 = 0;
        let mut prev_node: u32 = vehicle.home_node;
        let mut seen = HashSet::new();

        let route: &Vec<u32> = &self.routes[vehicle_id];

        // Simulate the route with insertions
        for i in 0..=route.len() {
            // Check if we need to insert pickup at this position
            if i == pickup_index {
                // Add travel cost/time to pickup location
                let travel = &travels[&(vehicle.index, prev_node, call.origin)];
                cost += travel.cost;
                time += travel.time;
                prev_node = call.origin;

                // Check pickup time window
                if call.pickup_end < time {
                    // info!("Vehicle was not in time for pickup for inserted call");
                    return Err(Infeasible::PickupExceeded);
                }
                if time < call.pickup_start {
                    time = call.pickup_start;
                }

                // Check capacity
                if capacity < call.size {
                    // info!("Vehicle ran out of capacity after inserting call");
                    return Err(Infeasible::CapacityExceeded);
                }
                capacity -= call.size;

                // Add loading cost/time
                cost += loading.origin_cost;
                time += loading.origin_time;

                // Mark call as seen
                seen.insert(call_to_insert);
            }

            // Check if we need to insert delivery at this position
            if i == delivery_index {
                // Must have seen pickup first
                if !seen.contains(&call_to_insert) {
                    return Err(Infeasible::DeliveryExceeded);
                }

                // Add travel cost/time to delivery location
                let travel = &travels[&(vehicle.index, prev_node, call.destination)];
                cost += travel.cost;
                time += travel.time;
                prev_node = call.destination;

                // Check delivery time window
                if call.delivery_end < time {
                    // info!("Vehicle was not in time for pickup for delivery call");
                    return Err(Infeasible::DeliveryExceeded);
                }
                if time < call.delivery_start {
                    time = call.delivery_start;
                }

                // Release capacity
                capacity += call.size;

                // Add unloading cost/time
                cost += loading.destination_cost;
                time += loading.destination_time;
            }

            // Process existing call at this position if any
            if i < route.len() {
                let current_call_index = route[i];

                // Skip if this is the same as our call_to_insert (shouldn't happen in insertion check)
                if current_call_index == call_to_insert {
                    continue;
                }

                let current_call = &calls[(current_call_index - 1) as usize];
                let current_loading = &loadings[&(vehicle.index, current_call_index)];

                if !seen.contains(&current_call_index) {
                    // Pickup operation
                    seen.insert(current_call_index);

                    // Travel to pickup
                    let travel = &travels[&(vehicle.index, prev_node, current_call.origin)];
                    cost += travel.cost;
                    time += travel.time;
                    prev_node = current_call.origin;

                    // Check pickup time window
                    if current_call.pickup_end < time {
                        // info!("Vehicle was not in time for pickup after inserting call");
                        return Err(Infeasible::PickupExceeded);
                    }
                    if time < current_call.pickup_start {
                        time = current_call.pickup_start;
                    }

                    // Check capacity
                    if capacity < current_call.size {
                        // info!("Vehicle ran out of capacity after inserting call");
                        return Err(Infeasible::CapacityExceeded);
                    }
                    capacity -= current_call.size;

                    // Add loading cost/time
                    cost += current_loading.origin_cost;
                    time += current_loading.origin_time;
                } else {
                    // Travel to delivery
                    let travel = &travels[&(vehicle.index, prev_node, current_call.destination)];
                    cost += travel.cost;
                    time += travel.time;
                    prev_node = current_call.destination;

                    // Check delivery time window
                    if current_call.delivery_end < time {
                        // info!("Vehicle was not in time for delivery after inserting call");
                        return Err(Infeasible::DeliveryExceeded);
                    }
                    if time < current_call.delivery_start {
                        time = current_call.delivery_start;
                    }

                    // Release capacity
                    capacity += current_call.size;

                    // Add unloading cost/time
                    cost += current_loading.destination_cost;
                    time += current_loading.destination_time;
                }
            }
        }
        // info!("Found feasible insert");

        Ok(cost)
    }

    pub fn find_best_insertion_for_vehicle(&self, vehicle_idx: usize, call: u32) -> Option<(usize, usize, u128)> {
        if !self.instance.compatibility[&((vehicle_idx + 1) as u32)].contains(&call) {
            return None;
        }

        // Store all feasible insertion positions with their costs
        let mut feasible_insertions = Vec::new();
        let num_positions = self.routes[vehicle_idx].len() + 1;
        let baseline_cost = match self.check_feasibility_one_vehicle(vehicle_idx) {
            Ok(cost) => cost,
            Err(_) => return None,
        };

        // Try all possible combinations of pickup and delivery positions
        for pickup_idx in 0..num_positions {
            // Delivery must come after pickup to maintain invariant
            for delivery_idx in 0..num_positions {
                // Check if this insertion combination is feasible
                match self.check_insertion_feasibility(vehicle_idx, call, pickup_idx, delivery_idx) {
                    Ok(cost) => {
                        let delta_cost = if cost > baseline_cost {
                            cost - baseline_cost
                        } else {
                            0 // Cost improvement is good! But shouldnt happen when inserting a call
                        };

                        feasible_insertions.push((pickup_idx, delivery_idx, delta_cost));
                    },
                    Err(_) => (),
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

    pub fn remove_call_from_vehicle(&mut self, vehicle_from: usize, call_idx: usize) -> u32 {
        let call = self.routes[vehicle_from].remove(call_idx);
        if let Some(index) = self.routes[vehicle_from].iter().position(|&x| x == call) {
            self.routes[vehicle_from].remove(index);
        } else {
            panic!("There were not two calls in vehicle")
        }

        call
    }

    pub fn get_random_vehicle(&self) -> Option<usize> {
        let rng = rng();

        let mut rng = rand::rng();
        let length = self.routes.len();

        // Find all non-empty vehicles
        let non_empty_vehicles: Vec<usize> =
            (0..length).filter(|&idx| !self.routes[idx].is_empty()).collect();

        // If no vehicles have calls, return the first regular vehicle
        // This shouldn't happen in practice, but provides a safety fallback
        if non_empty_vehicles.is_empty() {
            return None; // Return first vehicle as a fallback
        }

        // Randomly select from non-empty vehicles
        Some(non_empty_vehicles[rng.random_range(0..non_empty_vehicles.len())])
    }

    pub fn insert_call(&mut self, vehicle_idx: usize, idx: usize, call: u32) {
        self.routes[vehicle_idx].insert(idx, call);
    }
}