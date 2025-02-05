use std::collections::HashSet;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Vehicle {
    pub index: u32,
    pub home_node: u32,
    pub start_time: u128,
    pub capacity: u128,
}

#[derive(Debug)]
pub struct Call {
    pub index: u32,
    pub origin: u32,
    pub destination: u32,
    pub size: u128,
    pub cost_outsource: u128,
    pub pickup_start: u128,
    pub pickup_end: u128,
    pub delivery_start: u128,
    pub delivery_end: u128
}

#[derive(Debug)]
pub struct Travel {
    pub vehicle_index: u32,
    pub origin: u32,
    pub destination: u32,
    pub time: u128,
    pub cost: u128
}

#[derive(Debug)]
pub struct Loading {
    pub vehicle_index: u32,
    pub call_index: u32,
    pub origin_time: u128,
    pub origin_cost: u128,
    pub destination_time: u128,
    pub destination_cost: u128
}

#[derive(Debug)]
pub struct Instance {
    pub num_nodes: u32,
    pub num_vehicles: u32,
    pub num_calls: u32,
    pub vehicles: Vec<Vehicle>,
    pub compatibility: HashMap<u32, HashSet<u32>>,
    pub valid_vehicles: HashMap<u32, Vec<u32>>,
    pub calls: Vec<Call>,
    pub travels: HashMap<(u32, u32, u32), Travel>,
    pub loadings: HashMap<(u32, u32), Loading>
}



pub fn read_file(file_path: &str) -> Instance {
    println!("In file {file_path}");
    let file = File::open(file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut compatibility: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut calls: Vec<Call> = Vec::new();
    let mut travels: HashMap<(u32, u32, u32), Travel> = HashMap::new();
    let mut loadings: HashMap<(u32, u32), Loading> = HashMap::new();

    let mut line= String::new();

    // Read number of nodes 
    reader.read_line(&mut line).expect("Couldnt read line");

    line = String::new();
    reader.read_line(&mut line).expect("Couldnt read line");
    
    let NUM_NODES: u32 = line.trim().parse::<u32>().unwrap();
    //println!("NUM NODES: {NUM_NODES}");


    reader.read_line(&mut line).expect("Couldnt read line");
    
    // Read number of vehicles
    line = String::new();
    reader.read_line(&mut line).expect("Couldnt read line");

    let NUM_VEHICLES = line.trim().parse::<u32>().unwrap();
    //println!("NUM VEHICLES: {NUM_VEHICLES}");

    reader.read_line(&mut line).expect("Couldnt read line");

    // Read and store vehicles
    for i in 0..NUM_VEHICLES {
        line = String::new();
        reader.read_line(&mut line).expect("Couldnt read line");

        let vals: Vec<u32> = line.trim().split(',').map(|x|->u32{x.parse().unwrap()}).collect();
        let vehicle = Vehicle {
            index: vals[0],
            home_node: vals[1],
            start_time: vals[2] as u128,
            capacity: vals[3] as u128,
        };

        vehicles.push(vehicle);
    }
    //println!("{vehicles:?}");

    // Read number of calls
    reader.read_line(&mut line).expect("Couldnt read line");

    line = String::new();
    reader.read_line(&mut line).expect("Couldnt read line");

    let NUM_CALLS = line.trim().parse::<u32>().unwrap();

    //println!("{NUM_CALLS}");

    reader.read_line(&mut line).expect("Couldnt read line");
    
    for i in 0..NUM_VEHICLES {
        line = String::new();
        reader.read_line(&mut line).expect("Couldnt read line");

        let vals: Vec<u32> = line.trim().split(',').map(|x|->u32{x.parse().unwrap()}).collect();
        let index = vals[0];
        let mut rest = HashSet::new(); 
        rest.extend(&vals[1..]);

        compatibility.insert(index, rest);
    }

    // Insert compatibility for outsource truck
    compatibility.insert(NUM_VEHICLES + 1, (1 .. NUM_CALLS + 1).collect());

    //println!("{compatibility:?}");

    reader.read_line(&mut line).expect("Couldnt read line");

    for i in 0..NUM_CALLS {
        line = String::new();
        reader.read_line(&mut line).expect("Couldnt read line");

        let vals: Vec<u128> = line.trim().split(',').map(|x|->u128{x.parse().unwrap()}).collect();

        let call = Call {
            index: vals[0] as u32,
            origin: vals[1] as u32,
            destination:vals[2] as u32,
            size: vals[3],
            cost_outsource:vals[4],
            pickup_start:vals[5],
            pickup_end:vals[6],
            delivery_start:vals[7],
            delivery_end:vals[8]
        };

        calls.push(call);
    }

    // Read travels
    
    reader.read_line(&mut line).expect("Couldnt read line");

    for _i in 0..NUM_NODES * NUM_NODES * NUM_VEHICLES {
        line = String::new();
        reader.read_line(&mut line).expect("Couldnt read line");

        let vals: Vec<u32> = line.trim().split(',').map(|x|->u32{x.parse().unwrap()}).collect();

        let travel = Travel {
            vehicle_index: vals[0],
            origin: vals[1],
            destination: vals[2],
            time: vals[3] as u128,
            cost: vals[4] as u128
        };

        travels.insert((vals[0], vals[1], vals[2]), travel);
    }
    //println!("len: {travels:?}");


    // Read loadings
    reader.read_line(&mut line).expect("Couldnt read line");

    for _i in 0..NUM_VEHICLES * NUM_CALLS{
        line = String::new();
        reader.read_line(&mut line).expect("Couldnt read line");

        let vals: Vec<i128> = line.trim().split(',').map(|x|->i128{x.parse().unwrap()}).collect();

        // If one element is -1 then not compatible -> skip
        if vals[2] == -1 {continue;}

        let loading = Loading {
            vehicle_index: vals[0] as u32,
            call_index: vals[1] as u32,
            origin_time: vals[2] as u128,
            origin_cost: vals[3] as u128,
            destination_time: vals[4] as u128,
            destination_cost: vals[5] as u128 
        };

        loadings.insert((vals[0] as u32, vals[1] as u32), loading);
    }
    //println!("{loadings:?}")

    // Compatibility formulated differently for faster lookup
    let mut valid_vehicles: HashMap<u32, Vec<u32>> = HashMap::new();
    for (vehicle, calls) in compatibility.iter() {
        for &call in calls {
            valid_vehicles.entry(call).or_insert_with(Vec::new).push(*vehicle);
        }
    }

    drop(reader);
    
    return Instance {
        num_nodes: NUM_NODES,
        num_vehicles: NUM_VEHICLES,
        num_calls: NUM_CALLS,
        vehicles: vehicles,
        compatibility: compatibility,
        valid_vehicles: valid_vehicles,
        calls: calls,
        travels: travels,
        loadings: loadings
    };
}
