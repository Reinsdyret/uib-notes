
use file_reader::parse_data::*;  // Import read_file function
use checker::checker::*;

fn main() {
    let solution: Vec<u32> = vec![0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7];
    
    let filename = "src/data/Call_7_Vehicle_3.txt";  // Update the path as needed
    
    let instance = read_file(filename);

    println!("{:?}", check_feasibility_and_get_cost(&instance, solution));
}
