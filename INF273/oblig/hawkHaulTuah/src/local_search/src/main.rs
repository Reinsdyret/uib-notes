mod operators;
use file_reader::parse_data::Instance;

fn main() {
    println!("Hello, world!");
    
    // Create a simple instance for testing
    let mut instance = Instance {
        num_nodes: 10,
        num_vehicles: 4,
        num_calls: 7,
        vehicles: Vec::new(),
        calls: Vec::new(),
        travels: std::collections::HashMap::new(),
        loadings: std::collections::HashMap::new(),
        compatibility: std::collections::HashMap::new(),
        valid_vehicles: std::collections::HashMap::new(),
    };
    
    let solution = vec![vec![4, 4, 2, 2], vec![5, 5, 7, 7], vec![1, 3, 1, 3], vec![6, 6]];
    
    // Use one_reinsert_greedy_insert instead of one_reinsert
    let result = operators::one_reinsert_greedy_insert(&instance, &solution);

    println!("{result:?}");
}
