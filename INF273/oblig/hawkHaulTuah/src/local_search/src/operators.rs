use std::{u32, usize};

use file_reader::parse_data::*;
use rand::prelude::*;
use checker::checker::*;

pub fn one_reinsert_compatibility(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
  let mut rng = rand::thread_rng();
  let mut solution = old_solution.clone();

  // Extract both calls
  let vehicle_from = get_random_vehicle(&solution);

  let call_idx = rng.gen_range(0..solution[vehicle_from].len());
  let call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);

  // Find random but compatible vehicle
  let vehicle_to_idx: u32 = get_random_compatible_vehicle(call, &instance);

  let vehicle_to = &mut solution[vehicle_to_idx as usize];

  // Insert calls in random position and run 1 iteration of 2-opt on both sides to determine where to put them
  let insert_idx1 = rng.gen_range(0..=vehicle_to.len());
  let insert_idx2 = rng.gen_range(0..=vehicle_to.len());
  vehicle_to.insert(insert_idx1, call);
  vehicle_to.insert(insert_idx2, call);

  return solution
}

pub fn one_reinsert_focus_dummy_random_compatibility(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
  let mut rng = rand::thread_rng();
  let call: u32;
  let mut solution = old_solution.clone();
  let mut vehicle_from: usize = solution.len() - 1;
  // Choosing call, prioritizing outsource vehicle but still random call.
  // Random vehicle if outsource vehicle is empty
  if !solution[vehicle_from].is_empty() {
      let call_idx = rng.gen_range(0..solution[vehicle_from].len());
      call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);

      // println!("Chose call {call}");
  } else {
      vehicle_from = get_random_vehicle(&solution);
      let call_idx = rng.gen_range(0..solution[vehicle_from].len());
      call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);
  }

  // Find random but compatible vehicle
  let vehicle_to_idx: u32 = get_random_compatible_vehicle(call, &instance);
  let vehicle_to = &mut solution[vehicle_to_idx as usize];

  // Insert calls in random position
  let insert_idx1 = rng.gen_range(0..=vehicle_to.len());
  let insert_idx2 = rng.gen_range(0..=vehicle_to.len());
  vehicle_to.insert(insert_idx1, call);
  vehicle_to.insert(insert_idx2, call);

  return solution
}

pub fn one_reinsert_compatibility_two_opt(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {

  let mut rng = rand::thread_rng();
  let mut solution = old_solution.clone();

  // Extract both calls
  let mut vehicle_from = rng.gen_range(0..solution.len());
  while solution[vehicle_from].is_empty() {
      vehicle_from = rng.gen_range(0..solution.len());
  }

  let call_idx = rng.gen_range(0..solution[vehicle_from].len());
  let call = solution[vehicle_from].remove(call_idx);

  if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
      solution[vehicle_from].remove(index);
  } else {
      panic!("There were not two calls in vehicle")
  }

  // Find random but compatible vehicle
  let mut vehicle_to_idx: u32 = rng.gen_range(0..solution.len()) as u32;
  while !instance.compatibility.contains_key(&(vehicle_to_idx + 1)) {
    vehicle_to_idx = rng.gen_range(0..solution.len()) as u32;
  }

  let vehicle_to = &mut solution[vehicle_to_idx as usize];

  // Insert calls in random position and run 1 iteration of 2-opt on both sides to determine where to put them
  let insert_idx1 = rng.gen_range(0..=vehicle_to.len());
  let insert_idx2 = rng.gen_range(0..=vehicle_to.len());
  vehicle_to.insert(insert_idx1, call);
  vehicle_to.insert(insert_idx2, call);

  let call1_idx: usize;
  let call2_idx: usize;
  let cost_switch_left: u128;
  let cost_switch_right: u128;
  // If they get placed beside each other only check on each side
  if insert_idx1 == insert_idx2 || insert_idx2 - insert_idx1 == 1{
    call1_idx = insert_idx1;
    call2_idx = insert_idx1 + 1;
  } else {
    call1_idx = insert_idx1.min(insert_idx2);
    call2_idx = insert_idx1.max(insert_idx2);
  }
  // Calculate switch left only
  if call1_idx != 0 { // No point in checking for switch with left call if there is no
    cost_switch_left = instance.travels[&(vehicle_to_idx+1, vehicle_to[call1_idx - 1], vehicle_to[call1_idx])].cost;
  }

  // Calculate switch right only
  if call2_idx != vehicle_to.len() - 1 {
    cost_switch_right = instance.travels[&(vehicle_to_idx+1, vehicle_to[call2_idx], vehicle_to[call2_idx + 1])].cost
  }

  // Calculate switch both
  // If there is one element between then we will have two switch situations: c,a,c can be -> acc or cca depending on what switch goes first
  // if call2_idx - call1_idx == 2 {

  // }

  return solution;
}

pub fn one_reinsert_random(old_solution: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let mut rng = rand::thread_rng();
  let mut solution = old_solution.clone();

  let mut vehicle_from = rng.gen_range(0..solution.len());
  while solution[vehicle_from].is_empty() {
      vehicle_from = rng.gen_range(0..solution.len());
  }

  let call_idx = rng.gen_range(0..solution[vehicle_from].len());
  let call = solution[vehicle_from].remove(call_idx);

  if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
      solution[vehicle_from].remove(index);
  } else {
      panic!("There were not two calls in vehicle")
  }

  let vehicle_to = rng.gen_range(0..solution.len());
  let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
  solution[vehicle_to].insert(insert_idx, call);
  let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
  solution[vehicle_to].insert(insert_idx, call);

  return solution;
}

pub fn one_reinsert_focus_dummy_random_feasible(old_solution: &Vec<Vec<u32>>, instance: &Instance) -> Vec<Vec<u32>> {
  let mut rng = rand::thread_rng();
  let call: u32;
  let mut solution = old_solution.clone();
  let mut vehicle_from: usize = solution.len() - 1;
  // Choosing call, prioritizing outsource vehicle but still random call.
  // Random vehicle if outsource vehicle is empty
  if !solution[vehicle_from].is_empty() && rand::random::<f64>() < 0.4 { // Value here changes improvement alot
      let call_idx = rng.gen_range(0..solution[vehicle_from].len());
      call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);

      // println!("Chose call {call}");
  } else {
      vehicle_from = get_random_vehicle(&solution);
      let call_idx = rng.gen_range(0..solution[vehicle_from].len());
      call = remove_call_from_vehicle(call_idx, vehicle_from, &mut solution);
  }

  let mut vehicle_to: Vec<u32>;
  let mut vehicle_to_idx: u32;
  let mut insert_idx1: usize;
  let mut insert_idx2: usize;
  let mut i: usize = 0;

  loop {
    // Find random but compatible vehicle
    vehicle_to_idx = get_random_compatible_vehicle(call, &instance);
    vehicle_to = solution[vehicle_to_idx as usize].clone();

    // Insert calls in random position
    insert_idx1 = rng.gen_range(0..=vehicle_to.len());
    insert_idx2 = rng.gen_range(0..=vehicle_to.len());
    vehicle_to.insert(insert_idx1, call);
    vehicle_to.insert(insert_idx2, call);

    if vehicle_to_idx as usize == solution.len() - 1 || check_feasibility_one_vehicle(&instance, &vehicle_to, vehicle_to_idx).1 { break; }
    if i >= 100 { break; }
    i += 1;
  }

  let vehicle_to = &mut solution[vehicle_to_idx as usize];
  vehicle_to.insert(insert_idx1, call);
  vehicle_to.insert(insert_idx2, call);

  return solution;
}

pub fn one_reinsert_focus_dummy_random(
  old_solution: &Vec<Vec<u32>>,
  instance: &Instance,
) -> Vec<Vec<u32>> {
  let mut rng = rand::thread_rng();
  let call: u32;
  let mut solution = old_solution.clone();
  let mut vehicle_from: usize = solution.len() - 1;
  // Choosing call, prioritizing outsource vehicle but still random call.
  // Random vehicle if outsource vehicle is empty
  if !solution[vehicle_from].is_empty() {
      let call_idx = rng.gen_range(0..solution[vehicle_from].len());
      call = solution[vehicle_from].remove(call_idx);

      // println!("Chose call {call}");
  } else {
      vehicle_from = rng.gen_range(0..solution.len());
      while solution[vehicle_from].is_empty() {
          vehicle_from = rng.gen_range(0..solution.len());
      }
      let call_idx = rng.gen_range(0..solution[vehicle_from].len());
      call = solution[vehicle_from].remove(call_idx);
  }

  if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
      solution[vehicle_from].remove(index);
  } else {
      panic!("There were not two calls in vehicle")
  }

  // Reinsert randomly
  let mut vehicle_to = rng.gen_range(0..solution.len());

  while !instance.compatibility[&((vehicle_to + 1) as u32)].contains(&call) {
      vehicle_to = rng.gen_range(0..solution.len());
  }
  // println!("Choose vehicle {vehicle_to}");

  // Insert into vehicle with least amount of calls
  let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
  solution[vehicle_to].insert(insert_idx, call);
  let insert_idx = rng.gen_range(0..=solution[vehicle_to].len());
  solution[vehicle_to].insert(insert_idx, call);

  return solution;
}

fn get_random_compatible_vehicle(call: u32, instance: &Instance) -> u32 {
  let mut rng = rand::thread_rng();
  let mut vehicle_to_idx: u32 = rng.gen_range(0..instance.num_vehicles) as u32;
  while !instance.compatibility[&(vehicle_to_idx + 1)].contains(&call) {
    vehicle_to_idx = rng.gen_range(0..instance.num_vehicles) as u32;
  }

  return vehicle_to_idx;
}

fn get_random_vehicle(solution: &Vec<Vec<u32>>) -> usize {
  let mut rng = rand::thread_rng();
  let mut vehicle = rng.gen_range(0..solution.len());
  while solution[vehicle].is_empty() {
      vehicle = rng.gen_range(0..solution.len());
  }

  return vehicle;
}

fn remove_call_from_vehicle(call_idx: usize, vehicle_from: usize, solution: &mut Vec<Vec<u32>>) -> u32{
  let call = solution[vehicle_from].remove(call_idx);

  if let Some(index) = solution[vehicle_from].iter().position(|&x| x == call) {
      solution[vehicle_from].remove(index);
  } else {
      panic!("There were not two calls in vehicle")
  }

  return call;
}

fn get_index_least_calls(solution: &Vec<Vec<u32>>) -> usize {
  let mut least_calls = usize::MAX;
  let mut least_index = usize::MAX;

  for (i, route) in solution[0..solution.len() - 1].iter().enumerate() {
      if route.len() < least_calls {
          least_calls = route.len();
          least_index = i;
      }
  }

  return least_index;
}
