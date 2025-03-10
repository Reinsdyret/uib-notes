use file_reader::parse_data::*;

fn main() {
    let solution: Vec<Vec<u32>> = vec![
        vec![],
        vec![],
        vec![],
        vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7],
    ];
    let filename = "src/data/Call_7_Vehicle_3.txt";

    let instance: Instance = read_file(filename);

    find_avg_delta(&solution, &instance, 0.80);

    println!("Hello, world!");
}
