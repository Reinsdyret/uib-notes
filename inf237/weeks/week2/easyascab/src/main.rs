use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();

    let _ = stdin.read_line(&mut line);

    let end_char: char = line.trim().split(' ').collect::<Vec<_>>()[0].chars().collect::<Vec<char>>()[0];
    let n:u32 = line.trim().split(' ').collect::<Vec<_>>()[1].parse().expect("fail parse");

    // Make graph with vertices a - end_char
    let mut g: HashMap<char, Vec<char>> = HashMap::new();

    for c in b'a'..= end_char as u8{
        g.insert(c as char, Vec::new());
    }

    for (key, value) in g.into_iter() {
        println!("{} / {:?}", key, value);
    }


    // Loop up to n and take input, then make the edges uv if two strings are for example xyut
    // xyvt, where x,y and t are arbitrary characters. If one string is longer than the other then
    // the ranking can still be used. Just not with the chars from string2 that dont match with any
    // in string1


    // Output:
    // If the graph has a cycle then output "IMPOSSIBLE"
    // If the graph does not have a path from a - end_char then output "AMBIGUOUS"
    // Else:
    //  Output the path from a - end_char


    println!("Hello, world!, {} {}", end_char, n);
}
