use std::io;


fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines();

    for line in lines.skip(1){
        let unwrapped:String = line.unwrap();
        if unwrapped == "" {
            break;
        }

        if unwrapped.contains("="){
            println!("skipped");
        } else {
            let chars:Vec<&str> = unwrapped.split('+').collect();
            let a: u32 = chars[0].parse().expect("a fail");
            let b: u32 = chars[1].parse().expect("b fail");

            println!("{}", (a + b));
        }
    }
}
