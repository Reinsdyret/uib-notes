use std::collections::HashSet;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::Rng;
use solution::solution::Solution;
use crate::operator::Operator;

pub struct TwoSwap;

impl Operator for TwoSwap {
    fn name(&self) -> &str { "TwoSwap" }

    fn apply<'a>(&self, solution: &Solution<'a>) -> Solution<'a> {
        let instance = solution.instance;
        let mut rng = rand::rng();
        let init_cost = match solution.check_feasibility() {
            Ok(cost) => cost,
            Err(e) => return Solution::new(solution.instance, solution.routes.clone())
        };

        // Get list of non-empty vehicles (excluding outsource)
        let non_empty_vehicles: Vec<usize> = (0..solution.routes.len() - 1)
            .filter(|&idx| !solution.routes[idx].is_empty())
            .collect();

        // Need at least 2 non-empty vehicles
        if non_empty_vehicles.len() < 2 {
            return Solution::new(solution.instance, solution.routes.clone());
        }

        // Try multiple combinations for better results
        let num_attempts = 100;

        for _ in 0..num_attempts {
            // Select two different vehicles with probability based on route length
            // Vehicles with more calls have higher probability of selection
            let mut vehicle_weights: Vec<f64> = non_empty_vehicles
                .iter()
                .map(|&idx| solution.routes[idx].len() as f64)
                .collect();

            if vehicle_weights.is_empty() || vehicle_weights.iter().sum::<f64>() == 0.0 {
                return Solution::new(solution.instance, solution.routes.clone());
            }

            let dist = match WeightedIndex::new(&vehicle_weights) {
                Ok(d) => d,
                Err(_) => return Solution::new(solution.instance, solution.routes.clone()),
            };

            let v1_idx_pos = dist.sample(&mut rng);
            let v1_idx = non_empty_vehicles[v1_idx_pos];

            // Temporarily remove the selected vehicle for second selection
            let mut remaining_vehicles = non_empty_vehicles.clone();
            remaining_vehicles.remove(v1_idx_pos);
            let v2_idx_pos = rng.random_range(0..remaining_vehicles.len());
            let v2_idx = remaining_vehicles[v2_idx_pos];

            // Get a set of candidate calls from each vehicle
            let unique_calls_v1: HashSet<u32> = HashSet::from_iter(solution.routes[v1_idx].iter().cloned());
            let unique_calls_v2: HashSet<u32> = HashSet::from_iter(solution.routes[v2_idx].iter().cloned());

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
            let mut new_solution = Solution::new(solution.instance, solution.routes.clone());

            // Find and remove call1 from vehicle1
            let call1_pos1 = new_solution.routes[v1_idx].iter().position(|&x| x == call1).unwrap();
            new_solution.remove_call_from_vehicle(v1_idx, call1_pos1);

            // Find and remove call2 from vehicle2
            let call2_pos1 = new_solution.routes[v2_idx].iter().position(|&x| x == call2).unwrap();
            new_solution.remove_call_from_vehicle(v2_idx, call2_pos1);

            // Find best insertion positions for each call in the other vehicle
            if let Some((p1_idx, d1_idx, _)) = new_solution.find_best_insertion_for_vehicle(v2_idx, call1)
            {
                // Insert call1 in vehicle2
                new_solution.routes[v2_idx].insert(p1_idx, call1);
                let adj_d1_idx = if d1_idx > p1_idx { d1_idx + 1 } else { d1_idx };
                new_solution.routes[v2_idx].insert(adj_d1_idx, call1);

                if let Some((p2_idx, d2_idx, _)) = new_solution.find_best_insertion_for_vehicle(v2_idx, call2)
                {
                    // Insert call2 in vehicle1
                    new_solution.routes[v1_idx].insert(p2_idx, call2);
                    let adj_d2_idx = if d2_idx > p2_idx { d2_idx + 1 } else { d2_idx };
                    new_solution.routes[v1_idx].insert(adj_d2_idx, call2);

                    // Check if new solution is feasible and better
                    match new_solution.check_feasibility() {
                        Ok(cost) => {
                            if cost < init_cost {
                                return new_solution;
                            }
                        },
                        Err(_) => ()
                    }

                }
            }
        }

        Solution::new(solution.instance, solution.routes.clone())
    }
}