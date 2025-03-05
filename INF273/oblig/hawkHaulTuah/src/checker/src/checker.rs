use file_reader::parse_data::{Call, Instance, Loading, Travel, Vehicle};
use log::{debug, error, info, log_enabled, warn, Level};
use std::collections::HashSet;

pub fn check_feasibility_and_get_cost(instance: &Instance, routes: &Vec<Vec<u32>>) -> (u128, bool) {
    let calls = &instance.calls;
    let travels = &instance.travels;
    let loadings = &instance.loadings;

    let mut cost: u128 = 0;

    //println!("{routes:?}");
    // Dont run last route as everything is outsourced
    for (i, route) in routes[0..routes.len() - 1].into_iter().enumerate() {
        // println!("{route:?}");
        let vehicle: &Vehicle = &instance.vehicles[i];
        let mut time: u128 = vehicle.start_time;
        let mut capacity: u128 = vehicle.capacity;

        let mut seen: HashSet<u32> = HashSet::new(); // Seen nodes, to know if pickup or delivery
        let mut prev_node: u32 = vehicle.home_node;

        // Normal processing for nodes that are by trucks
        for call_index in route.into_iter() {
            // Calculate running cost.
            // Verify if end time is less than current time then not feasible
            // Verity if capacity is over limit / < 0. Remember to add and remove capacity when loading and unloading.
            // One case for pickup

            if !loadings.contains_key(&(vehicle.index, *call_index)) {
                info!("Call not compatible with vehicle");
                return (0, false);
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
                    info!("Vehicle did not have time for pickup");
                    return (0, false);
                }

                // If time < pickup_start -> wait
                if time < call.pickup_start {
                    time = call.pickup_start;
                }

                // Check if space and pickup package
                if capacity < call.size {
                    info!("Vehicle ran out of capacity");
                    return (0, false);
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
                    info!("Vehicle did not have time for delivery");
                    return (0, false);
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
    }

    // Add all outsourced calls to cost
    let mut seen: HashSet<u32> = HashSet::new(); // Seen nodes
    for call_index in &routes[routes.len() - 1] {
        if seen.contains(call_index) {
            continue;
        }
        seen.insert(*call_index);
        let call = &calls[(call_index - 1) as usize];
        cost += call.cost_outsource;
    }

    return (cost, true);
}

pub fn check_feasibility_one_vehicle(
    instance: &Instance,
    route: &Vec<u32>,
    vehicle_id: usize,
) -> (u128, bool) {
    let calls = &instance.calls;
    let travels = &instance.travels;
    let loadings = &instance.loadings;

    let mut cost: u128 = 0;

    let vehicle: &Vehicle = &instance.vehicles[vehicle_id];
    let mut time: u128 = vehicle.start_time;
    let mut capacity: u128 = vehicle.capacity;

    let mut seen: HashSet<u32> = HashSet::new(); // Seen nodes, to know if pickup or delivery
    let mut prev_node: u32 = vehicle.home_node;

    // Normal processing for nodes that are by trucks
    for call_index in route.into_iter() {
        // Calculate running cost.
        // Verify if end time is less than current time then not feasible
        // Verity if capacity is over limit / < 0. Remember to add and remove capacity when loading and unloading.
        // One case for pickup

        if !loadings.contains_key(&(vehicle.index, *call_index)) {
            return (0, false);
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
                return (0, false);
            }

            // If time < pickup_start -> wait
            if time < call.pickup_start {
                time = call.pickup_start;
            }

            // Check if space and pickup package
            if capacity < call.size {
                return (0, false);
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
                return (0, false);
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

    return (cost, true);
}

pub fn check_insertion_feasibility(
    instance: &Instance,
    route: &Vec<u32>,
    vehicle_id: usize,
    call_to_insert: u32,
    pickup_index: usize,
    delivery_index: usize,
) -> (u128, bool) {
    let calls = &instance.calls;
    let travels = &instance.travels;
    let loadings = &instance.loadings;
    let vehicle = &instance.vehicles[vehicle_id];

    // Check if vehicle is compatible with call
    if !loadings.contains_key(&(vehicle.index, call_to_insert)) {
        return (0, false);
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
                info!("Vehicle was not in time for pickup for inserted call");
                return (0, false);
            }
            if time < call.pickup_start {
                time = call.pickup_start;
            }

            // Check capacity
            if capacity < call.size {
                info!("Vehicle ran out of capacity after inserting call");
                return (0, false);
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
                return (0, false);
            }

            // Add travel cost/time to delivery location
            let travel = &travels[&(vehicle.index, prev_node, call.destination)];
            cost += travel.cost;
            time += travel.time;
            prev_node = call.destination;

            // Check delivery time window
            if call.delivery_end < time {
                info!("Vehicle was not in time for pickup for delivery call");
                return (0, false);
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
                    info!("Vehicle was not in time for pickup after inserting call");
                    return (0, false);
                }
                if time < current_call.pickup_start {
                    time = current_call.pickup_start;
                }

                // Check capacity
                if capacity < current_call.size {
                    info!("Vehicle ran out of capacity after inserting call");
                    return (0, false);
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
                    info!("Vehicle was not in time for delivery after inserting call");
                    return (0, false);
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
    info!("Found feasible insert");

    (cost, true)
}
