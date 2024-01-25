use std::io;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp;

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();

    let _ = stdin.read_line(&mut line);

    let end_char: char = line.trim().split(' ').collect::<Vec<_>>()[0].chars().collect::<Vec<char>>()[0];
    let n:u32 = line.trim().split(' ').collect::<Vec<_>>()[1].parse().expect("fail parse");

    // Make graph with vertices a - end_char
    let mut g: HashMap<char, Vec<char>> = HashMap::new();
    let mut g_reversed: HashMap<char, Vec<char>> = HashMap::new();

    for c in b'a'..= end_char as u8{
        g.insert(c as char, Vec::new());
        g_reversed.insert(c as char, Vec::new());
    }

    

    // Loop up to n and take input, then make the edges uv if two strings are for example xyut
    // xyvt, where x,y and t are arbitrary characters. If one string is longer than the other then
    // the ranking can still be used. Just not with the chars from string2 that dont match with any
    // in string1

    let mut prev_line = String::new();
    let mut new_line = String::new();
    let _ = stdin.read_line(&mut prev_line);

    for _ in 0..n-1 {
        let _ = stdin.read_line(&mut new_line);

        //println!("{} against {}", prev_line.trim(), new_line.trim());

        // Loop for i: 0 to min(prev_line.len(), new_line.len())
        // Check if prev_line[i] == new_line[i]. If not then create edge prev_line[i] -> new_line[i]
        for i in 0..cmp::min(prev_line.len(), new_line.len()) {
            let char1 = prev_line.as_bytes()[i];
            let char2 = new_line.as_bytes()[i];
            if char1 == char2 {
                continue;
            }

            //println!("{} won against {}", char1 as char, char2 as char);

            g.get_mut(&(char1 as char)).expect("Could not find char").push(char2 as char);

            g_reversed.get_mut(&(char2 as char)).expect("Could not find char").push(char1 as char);
            break;
        }

        prev_line = new_line.clone();
        new_line = "".to_string();
    }

    print_hashmap(&g);
    print_hashmap(&g_reversed);




    // Output:
    // If the graph has a cycle then output "IMPOSSIBLE"

    // Using DFS, non-recursively, to traverse through graph. If I hit any nodes I have hit before,
    // there is a cycle.
    // Use reversed graph to find root

    //let 

    //has_cycle(&g, )

    // If the graph does not have a path from a - end_char then output "AMBIGUOUS"
    // Else:
    //  Output the path from a - end_char

}

fn print_hashmap(map: &HashMap<char, Vec<char>>) {
    for (key, value) in map {
        println!("{} / {:?}", key, value);
    }
}

fn has_cycle(graph: &HashMap<char, Vec<char>>, root: char) -> bool{
    // Function taking in hashmap as a graph. Traverses using dfs and returns true if a cycle is
    // found and false otherwise.

    let mut to_visit: VecDeque<char> = VecDeque::from([root]);
    let mut visited = HashSet::from([root]);
    let mut current_node = root.clone();
    let mut neighbours: VecDeque<char> = VecDeque::new();

    while to_visit.len() != 0 {
        current_node = to_visit.pop_front().expect("Empty to_visit").clone();

        neighbours = graph.get(&root).expect("no key").clone().into();

        to_visit.append(&mut neighbours);

        for v in &mut neighbours {
            println!("{}", v);
        }
    }

    return false;
}
