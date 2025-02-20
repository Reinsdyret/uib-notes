use std::io;
use std::collections::{HashSet, VecDeque};

fn main() {
    // Take in start and goal input
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let start_state: Vec<u8>= line.trim().to_string().chars().map(|c| c as u8).collect();

    line = "".to_string();
    let _ = io::stdin().read_line(&mut line);
    let mut end_state: Vec<u8> = line.trim().to_string().chars().map(|c| c as u8).collect();

    // Use a VecDeque to keep track of states to visit
    let mut to_visit: Vec<Vec<u8>> = [start_state.clone()].into();
    // Use a HashSet to keep track of visited states
    let mut visited: HashSet<Vec<u8>> = HashSet::new();
    // Use a mutable usize to keep track of steps
    let mut steps: usize = 0;

    let mut complete: bool = false;

    let mut next_level: Vec<Vec<u8>> = [].into(); 

    if start_state == end_state  {complete = true}

    while !to_visit.is_empty() && !complete {

        while let Some(state) = to_visit.pop() {

            if state == end_state {
                complete = true;
                break;
            }

            let cloned_state = state.clone();

            // then generate all possible states from that states.
            for i in 0..8 {
                let next_state = click(cloned_state.clone(), i);

                if !visited.contains(&next_state) && !to_visit.contains(&next_state) {
                    visited.insert(next_state.clone());
                    next_level.push(next_state);
                } 

                //if visited.contains(&next_state) {
                //    println!("Seen {}", next_state.clone().into_iter().map(|c| c as char).collect::<String>());
                //}
            }
        }

        to_visit.append(&mut next_level);
        next_level = Vec::new();

        if !complete {
            steps += 1;
            println!("{}", steps);
        }
    }

    println!("{}", steps);
    
}

fn rotate_at(mut word: Vec<u8>, index: usize) -> Vec<u8>{
    //let mut to_change: u8 = word.get_mut(index).unwrap();
    word[index] = (word.get(index).unwrap() - (64 as u8)) % (6 as u8) + (65 as u8);
    return word;
    // Math created by chatgpt but it works
    //to_change = ((to_change + (1 as u8) - (65 as u8))) % (6 as u8) + 65 as u8;
}

fn A(mut word: Vec<u8>, index: usize) -> Vec<u8> {
    if index == 0 {
        rotate_at(word, index + 1)
    } else if index == word.len() - 1 {
        rotate_at(word, index - 1)
    } else {
        rotate_at((rotate_at(word, (index - 1))), index + 1)
    }
}

fn B(mut word: Vec<u8>, index: usize) -> Vec<u8> {
    if index == word.len() - 1 || index == 0{
        word
    } else {
        word[index + 1] = word[index - 1].clone();

        word
    }
}

fn C(mut word: Vec<u8>, index: usize) -> Vec<u8> {
    rotate_at(word, 7 - index)
}

fn D(mut word: Vec<u8>, index: usize) -> Vec<u8> {
    // We know that a string is always of len = 8 so we canjust harcode values
    // Middles are 3 and 4 so if smaller than 4 do 0-3 and if greater than 3 do 4-7


    if index < 4 {
        for i in 0..4 {
            if i == index {continue;}
            //temp = curr.clone();
            word = rotate_at(word, i);
        }
    } else {
        for i in 4..8 {
            if i == index {continue;}
            word = rotate_at(word, i);
        }
    }
    word
}


fn E(mut word: Vec<u8>, index: usize) -> Vec<u8> {
     if index == 0 || index == 7 {
         return word;
     }

     if index < 4 {
         rotate_at(rotate_at(word, 0), index + index)
     } else {
         let diff = word.len() - index;
         rotate_at(rotate_at(word, 7), index - diff)
     }
}

fn F(mut word: Vec<u8>, index: usize) -> Vec<u8>{
    if (index + 1) % 2 == 0 {
        let new_index = (index as u32) / 2;
        rotate_at(word, new_index as usize)
    } else {
        let new_index = ((index + 9) as u32) / 2;
        rotate_at(word, new_index as usize)
    }
}

fn click(mut state: Vec<u8>, i: usize) -> Vec<u8>{
    match state[i] as char {
                'A' => return A(state, i),
                'B' => return B(state, i),
                'C' => return C(state, i),
                'D' => return D(state, i),
                'E' => return E(state, i),
                'F' => return F(state, i),
                _ => {panic!("Did not match char");}
    };
}
