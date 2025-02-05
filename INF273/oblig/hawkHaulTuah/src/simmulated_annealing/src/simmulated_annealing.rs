use local_search::local_search::*;
use file_reader::parse_data::Instance;



pub fn find_avg_delta(init_solution: &Vec<Vec<u32>>, instance: &Instance, prob: f64) -> (f64, Vec<Vec<u32>>) {
    let mut incumbent: Vec<Vec<u32>> = Vec::with_capacity(init_solution.len());
    let mut best_solution: Vec<Vec<u32>> = Vec::with_capacity(init_solution.len());
    incumbent.extend_from_slice(&init_solution);
    best_solution.extend_from_slice(&init_solution);

    let mut new_solution: Vec<Vec<u32>>;

    for w in 1 ..= 100 {
        new_solution = one_reinsert_focus_dummy_random()
    }


    return (0.0, incumbent);
}
