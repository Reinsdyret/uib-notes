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
