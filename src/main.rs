use std::io;
use rand::Rng;

fn main() {
    println!("Enter your name :");
    let mut inp = String::new();
    let mut mat: Vec<Vec<char>> = vec![vec!['.'; 3]; 3];
    let mut example: Vec<Vec<usize>> = vec![vec![0; 3]; 3];
    for i in 0..example.len() {
        for j in 0..example[0].len() {
            example[i][j] += i*2+j+1;
        }
    }
    println!("To play entire the number representing the square you want to place your X.");
    println!("You are X's! Play the first move!");
    println!("{}", random_move(mat));
    let mut readin = io::stdin().read_line(&mut inp);
    println!("{}", inp);
    let my_int = my_string.parse::<i32>().unwrap();




}


fn print_board(vec: Vec<Vec<char>>) {
    for i in 0..vec.len() {
        vec[i].iter().for_each(|val| print!("{} ", val));
        println!("");
    }
}

fn random_move(vec: Vec<Vec<char>>) -> usize {
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