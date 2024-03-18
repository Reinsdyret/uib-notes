use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::hash::Hash;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    B, // Black knight
    W, // White knight
    E, // Empty
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Board {
    board: Vec<Vec<Square>>,
    empty_square: (i32, i32),
}

impl Board {
    fn new(arr: Vec<Vec<Square>>) -> Board {
        let mut e_x: i32 = 0;
        let mut e_y: i32 = 0;

        for y in 0..arr.len() {
            for x in 0..arr[y].len() {
                if arr[y][x] == Square::E {
                    e_x = x as i32;
                    e_y = y as i32;
                }
            }
        }
        Board {
            board: arr,
            empty_square: (e_x, e_y),
        }
    }

    fn next_boards(&self) -> Vec<Board> {
        let (x, y) = self.empty_square;

        let cases: Vec<(i32, i32)> = vec![
            (x + 1, y + 2),
            (x + 2, y + 1),
            (x + 2, y - 1),
            (x + 1, y - 2),
            (x - 1, y - 2),
            (x - 2, y - 1),
            (x - 2, y + 1),
            (x - 1, y + 2),
        ];

        let mut next_boards: Vec<Board> = Vec::new();

        for (nx, ny) in cases {
            if nx >= 0 && nx < 5 && ny >= 0 && ny < 5 {

                // TODO: Might need to fix this copying
                let mut new_board: Vec<Vec<Square>> = Vec::new();

                for row in &self.board {
                    new_board.push(row.clone());
                }

                new_board[y as usize][x as usize] = new_board[ny as usize][nx as usize].clone();

                new_board[ny as usize][nx as usize] = Square::E;

                next_boards.push(Board::new(new_board));
            }
        }

        return next_boards;
    }

    fn is_finish_conf(&self) -> bool {
        let goal: Vec<Vec<Square>> = vec![
            vec![Square::B, Square::B, Square::B, Square::B, Square::B],
            vec![Square::W, Square::B, Square::B, Square::B, Square::B],
            vec![Square::W, Square::W, Square::E, Square::B, Square::B],
            vec![Square::W, Square::W, Square::W, Square::W, Square::B],
            vec![Square::W, Square::W, Square::W, Square::W, Square::W],
        ];

        return self.board == goal;
    }
}


fn bfs_board_max_depth(board: Board) -> i32 {
    let mut queue: VecDeque<(Board, i32)> = VecDeque::new();
    queue.push_back((board.clone(), 0));

    let mut visited: HashSet<Board> = HashSet::new();
    visited.insert(board);

    while let Some((curr_board, depth)) = queue.pop_front() {
        if curr_board.is_finish_conf() {
            return depth;
        }

        if depth >= 11 {
            continue;
        }

        for next_board in curr_board.next_boards() {
            if visited.insert(next_board.clone()) {
                queue.push_back((next_board, depth + 1));
            }
        }
    }

    return -1;
}

fn main() {
    //let (stdin, stdout) = (io::stdin(), io::stdout());

    let [n]: [i32; 1] = get_n_i32().try_into().expect("Couldnt parse vec as arr");

    for _ in 0..n {
        let mut b: Vec<Vec<Square>> = Vec::new();

        for _ in 0..5 {
            let row = get_square_row();
            b.push(row);
        }

        let board = Board::new(b);
        //println!("{:?}", board.board);

        let res = bfs_board_max_depth(board);

        if res == -1 {
            println!("Unsolvable in less than 11 move(s).");
        } else {
            println!("Solvable in {} move(s).", res);
        }
    }
}

fn get_n_i32() -> Vec<i32> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line
        .trim()
        .split(' ')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<i32>>();
}

fn get_square_row() -> Vec<Square> {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    return line
        .trim()
        .chars()
        .map(|c| {
            if c == '1' {
                Square::B
            } else if c == ' ' {
                Square::E
            } else {
                Square::W
            }
        })
        .collect::<Vec<Square>>();
}
