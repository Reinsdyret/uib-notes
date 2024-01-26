use std::io;
use std::collections::{HashSet, VecDeque};

fn main() {
    // Take in start and goal input
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let start_state: String = line.trim().to_string();

    line = "".to_string();
    let _ = io::stdin().read_line(&mut line);
    let end_state: String = line.trim().to_string();

    println!("{}", B(&end_state, 6));

    // Use a VecDeque to keep track of states to visit
    let mut to_visit: VecDeque<String> = [start_state.clone()].into();
    // Use a HashSet to keep track of visited states
    let mut visited: HashSet<String> = HashSet::new();
    // Use a mutable usize to keep track of steps
    let mut steps: usize = 0;
    // First visit the state in deque.pop_first(), increase step variable,
    // then generate all possible states from that states.
    // For each generated states 
    //  if it is a the goal state -> finish
    //  if it is in the visited set then skip
    //  else: add it do the deque
    
}

fn rotate_at(word: &str, index: usize) -> String{
    let mut letters: Vec<u8> = word.chars().map(|c| c as u8).collect();
    let mut to_change: &mut u8 = letters.get_mut(index).unwrap();
    // Math created by chatgpt but it works
    *to_change = ((*to_change + 1 - 65) % 6) + 65;
    return String::from_iter(letters.into_iter().map(|b| b as char));
}

fn A(word: &str, index: usize) -> String {
    if index == 0 {
        rotate_at(word, index + 1)
    } else if index == word.len() - 1 {
        rotate_at(word, index - 1)
    } else {
        rotate_at(&(rotate_at(word, (index - 1).clone())), index + 1)
    }
}

fn B(word: &str, index: usize) -> String {
    if index == word.len() - 1 {
        word.to_string()
    } else {
        let mut letters: Vec<char> = word.chars().collect();
        letters[index + 1] = letters[index - 1].clone();

        return String::from_iter(letters.into_iter());
    }
}

fn C(word: &str, index: usize) -> String {
    rotate_at(word, 8 - index)
}
