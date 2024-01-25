use std::io;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp;
use std::process;

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();

    let _ = stdin.read_line(&mut line);

    let end_char: char = line.trim().split(' ').collect::<Vec<_>>()[0].chars().collect::<Vec<char>>()[0];
    let n:u32 = line.trim().split(' ').collect::<Vec<_>>()[1].parse().unwrap();

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
        new_line = new_line.trim().to_string();
        prev_line = prev_line.trim().to_string();

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

    //print_hashmap(&g);
    //print_hashmap(&g_reversed);




    // Output:
    // If the graph has a cycle then output "IMPOSSIBLE"


    let leaves = find_leaves(&g_reversed);

    match leaves.len() {
        // If there is no leaves then there is no end and there is cycle
        0 => println!("IMPOSSIBLE"),
        1 => println!("{}", find_path(&g, leaves[0], n)),
        // If the graph has multiple leaves, it does not have a clear order
        _ => println!("AMBIGUOUS"),
    }

}

fn print_hashmap(map: &HashMap<char, Vec<char>>) {
    for (key, value) in map {
        println!("{} / {:?}", key, value);
    }
}

fn find_leaves(tree: &HashMap<char, Vec<char>>) -> Vec<char> {
    let mut leaves: Vec<char> = Vec::new();

    for (node, edges) in tree {
        if edges.len() == 0 {
            leaves.push(*node)
        }
    }

    return leaves;
}

fn find_path(tree: &HashMap<char, Vec<char>>, root: char, n: u32) -> String {
    let mut neighbours: Vec<char> = tree.get(&root).expect("no neighbours").clone();

    let mut path = String::new();
    path.push(root);

    while neighbours.len() > 0 {
        let neighbour = &neighbours[0];
        path.push(*neighbour);
        neighbours = tree.get(neighbour).expect("no neighbours of neighbour").clone();
    }

    if (path.len() as u32) < n {
        return "AMBIGUOUS".to_string(); // TODO: Impossible or AMBIGUOUS here? I have no clue :)
    }
    return path;
}
