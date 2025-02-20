use std::io;

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    let _ = stdin.read_line(&mut line);

    let n: i32 = line.trim().parse().expect("Expected int on first line");

    for _ in 0..n {
        //line = String::new();
        //let _ = stdin.read_line(&mut line);
        line = get_line();
        let items: Vec<&str> = line.split(' ').collect();
        let width: i32 = items[0].parse().expect("Expected int");
        let height: i32 = items[1].parse().expect("Expected int");
        let person: &str = items[2];

        let log2_width:u32 = i32::ilog2(width);
        let log2_height:u32 = i32::ilog2(height);

        if person == "Vicky" {
            if log2_width > log2_height {
                println!("Vicky can win");
            } else {
                println!("Vicky cannot win");
            }
        } else {
            if log2_height > log2_width {
                println!("Harry can win");
            } else {
                println!("Harry cannot win")
            }
        }
    }
}

fn get_line() -> String {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line.trim().to_string();
}