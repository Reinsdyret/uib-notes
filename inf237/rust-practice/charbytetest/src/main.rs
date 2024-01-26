fn main() {
    println!("Hello, world!");
    println!("{:?}", ("A".to_string().as_bytes()[0] + (71 as u8)) % 70);
    println!("{:?}", ("B".to_string().as_bytes()[0]));
}
