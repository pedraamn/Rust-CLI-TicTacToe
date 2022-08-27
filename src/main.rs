use std::io;
use std::fmt::Display;
use rand::Rng;
use std::{thread, time};
use std::collections::HashSet;

fn main() {
    let sleep_interval = time::Duration::from_millis(900);
    let winner: String;
    let mut occupied: HashSet<usize> = HashSet::new();
    let mut board: Vec<Vec<char>> = vec![vec!['.'; 3]; 3];
    let mut example: Vec<Vec<usize>> = vec![vec![0; 3]; 3];
    for i in 0..example.len() {
        for j in 0..example[0].len() {
            example[i][j] += i*3+j+1;
        }
    }
    print_board(&example);
    println!("To play entire the number representing the square you want to place your X.");
    println!("You are X's. Play the first move.");
    loop {
        let mut inp = String::new();
        let _ = io::stdin().read_line(&mut inp);
        let mut mv = parse_move(inp);
        loop {
            if occupied.contains(&mv) {
                println!("Square is already occupied. Please select an empty square.");
            } else if mv == 0 {
                print_board(&example);
                println!("Invalid input. Please enter a digit 1-9 representing the square on the board.");
            } else {
                occupied.insert(mv);
                break;
            }
            inp = String::new();
            _ = io::stdin().read_line(&mut inp);
            mv = parse_move(inp);
        }
        let (mut row, mut col) = get_row_col(mv-1);
        board[row][col] = 'X';
        if has_won(&board, mv) {
            winner = String::from("You win! Congrations!");
            break;
        }
        print_board(&board);
        println!("Opponent thinking...");
        thread::sleep(sleep_interval);
        let opp_move = random_move(&board);
        occupied.insert(opp_move+1);
        (row, col) = get_row_col(opp_move);
        board[row][col] = 'O';
        if has_won(&board, opp_move+1) {
            winner = String::from("Defeat! Opponent has won!");
            break;
        }
        print_board(&board);
        println!("Opponent has moved. Your turn.")
    }
    print_board(&board);
    println!("{}", winner)

}

fn print_board<T: Display>(vec: &Vec<Vec<T>>) {
    println!("-----");
    for i in 0..vec.len() {
        vec[i].iter().for_each(|val| print!("{} ", val));
        println!("");
    }
    println!("-----");
}

fn parse_move(mv: String) -> usize {
    if mv.len() != 2 {
        return 0;
    }
    let mut res = mv.as_bytes()[0];
    match res {
        49..=57 => {
            res -= 48;
            return res as usize;
        }
        _ => {
            return 0
        }
    }
}

fn has_won(vec: &Vec<Vec<char>>, last_move: usize) -> bool {
    let (row, col, diag);
    if last_move == 5 {
        return check_three(vec, 1, 1) || check_three(vec, 2, 1) || check_three(vec, 3, 0) || check_three(vec, 3, 1);
    } else if last_move%2 == 1 {
        row = if last_move < 4 {0} else {2};
        col = if last_move%3 == 0 {2} else {0};
        diag = if last_move==1 || last_move==9 {0} else {1};
        return check_three(vec, 1, row) || check_three(vec, 2, col) || check_three(vec, 3, diag);
    } else {
        if last_move == 2 {
            row = 0;
            col = 1;
        } else if last_move == 4 {
            row = 1;
            col = 0;
        } else if last_move == 6 {
            row = 1;
            col = 2;
        } else {
            row = 2;
            col = 1;
        }
        return check_three(vec, 1, row) || check_three(vec, 2, col);
    }

}

fn check_three(vec: &Vec<Vec<char>>, orth: usize, start: usize) -> bool {
    if start >= 3 {
        return false;
    }
    if orth == 1 {
        return vec[start][0] == vec[start][1] && vec[start][1] == vec[start][2]
    } else if orth == 2 {
        return vec[0][start] == vec[1][start] && vec[1][start] == vec[2][start]
    } else if orth == 3 {
        if start == 0 { 
            return vec[0][0] == vec[1][1] && vec[1][1] == vec[2][2]
        } else {
            return vec[0][2] == vec[1][1] && vec[1][1] == vec[2][0]
        }
    } else {
        return false;
    }
}

fn random_move(vec: &Vec<Vec<char>>) -> usize {
    let mut squares: Vec<usize> = Vec::new();
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i][j] == '.' {
                squares.push(i*3+j);
            }
        }
    }
    let rand_idx = rand::thread_rng().gen_range(0..squares.len());
    return squares[rand_idx];
}

fn get_row_col(square: usize) -> (usize, usize) {
    let (row, col) = (square/3, square%3);
    return (row, col);
}