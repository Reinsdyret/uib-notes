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


    // Use a VecDeque to keep track of states to visit
    let mut to_visit: VecDeque<String> = [start_state.clone()].into();
    // Use a HashSet to keep track of visited states
    let mut visited: HashSet<String> = HashSet::new();
    // Use a mutable usize to keep track of steps
    let mut steps: usize = 0;

    let mut complete: bool = false;

    if start_state == end_state  {complete = true}

    while !to_visit.is_empty() && !complete {
        let level_size = to_visit.len(); // num of states in lvl

        for _ in 0..level_size {
            let state = to_visit.pop_front().unwrap();

            if state == end_state {
                complete = true;
                break;
            }

            visited.insert(state.clone());

            // then generate all possible states from that states.
            for i in 0..8 {
                let next_state = click(&state, i);

                if !visited.contains(&next_state) && !to_visit.contains(&next_state) {
                    to_visit.push_back(next_state);
                } 
            }
        }

        if !complete {
            steps += 1;
        }
    }

    println!("{}", steps);
    
}

fn rotate_at(word: &str, index: usize) -> String{
    let mut letters: Vec<u8> = word.chars().map(|c| c as u8).collect();
    let mut to_change: &mut u8 = letters.get_mut(index).unwrap();
    // Math created by chatgpt but it works
    *to_change = ((*to_change + 1 - 65) % 6) + 65;
    return letters.into_iter().map(|b| b as char).collect();
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
    if index == word.len() - 1 || index == 0{
        word.to_string()
    } else {
        let mut letters: Vec<char> = word.chars().collect();
        letters[index + 1] = letters[index - 1].clone();

        return letters.into_iter().collect();
    }
}

fn C(word: &str, index: usize) -> String {
    rotate_at(word, 7 - index)
}

fn D(word: &str, index: usize) -> String {
    // We know that a string is always of len = 8 so we canjust harcode values
    // Middles are 3 and 4 so if smaller than 4 do 0-3 and if greater than 3 do 4-7

    let mut curr = word.clone().to_string();
    //let mut temp = "".to_string();

    if index < 4 {
        for i in 0..4 {
            if i == index {continue;}
            //temp = curr.clone();
            curr = rotate_at(&curr, i);
        }
    } else {
        for i in 4..8 {
            if i == index {continue;}
            curr = rotate_at(&curr, i);
        }
    }

    return curr;
}


fn E(word: &str, index: usize) -> String {
     if index == 0 || index == 7 {
         return word.to_string();
     }

     if index < 4 {
         rotate_at(&rotate_at(word, 0), index + index)
     } else {
         let diff = word.len() - index;
         rotate_at(&rotate_at(word, 7), index - diff)
     }
}

fn F(word: &str, index: usize) -> String {
    if (index + 1) % 2 == 0 {
        let new_index = (index as u32) / 2;
        rotate_at(word, new_index as usize)
    } else {
        let new_index = ((index + 9) as u32) / 2;
        rotate_at(word, new_index as usize)
    }
}

fn click(state: &str, i: usize) -> String{
    let state_b = state.as_bytes();
    match state_b[i] as char {
                'A' => return A(state, i),
                'B' => return B(state, i),
                'C' => return C(state, i),
                'D' => return D(state, i),
                'E' => return E(state, i),
                'F' => return F(state, i),
                _ => {panic!("Did not match char");}
    };
    return "".to_string();
}
