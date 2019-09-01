// 30/08/2019
//
// The N-Queens puzzle (wiki) is the problem of placing N queens on an N×N
// chessboard such that no two queens attack each other. Given an integer N,
// return all distinct solutions to the N-Queens puzzle. Each solution
// contains a distinct board configuration of the N-Queens' placement, where
// 'Q' and '.' both indicate a queen and an empty space respectively. 
//
// Example:
// Input: 4
//
//  Output: 
//  [
//     [".Q..",  // Solution 1
//      "...Q",
//      "Q...",
//      "..Q."],
//
//     ["..Q.",  // Solution 2
//      "Q...",
//      "...Q",
//      ".Q.."]
// ]
//  Why?: There exist two distinct solutions to the 4-queens puzzle as shown
//  above.
use std::cmp::{min,max};

fn dfs(n: u32, board: &mut Vec<u32>, solutions: &mut Vec<Vec<u32>>) {
    if board.len() as u32 == n{
        solutions.push(board.clone());
        return;
    }

    'outer: for pos_i in 0..n {
        for (j, &pos_j) in board.iter().enumerate() {
            let pos_distance: usize = (max(pos_i, pos_j) - min(pos_i, pos_j)) as usize;

            if (board.len() - j) == pos_distance || pos_i == pos_j {
                continue 'outer;
            }
        }

        board.push(pos_i);
        dfs(n, board, solutions);
        board.pop();
    }
}

fn get_queens_puzzle_soultions(n: u32) -> Vec<Vec<u32>> {
    let mut solutions = Vec::new();
    let mut board = Vec::new();
    dfs(n, &mut board, &mut solutions);
    return solutions;
}

fn pp_solution(solution: &Vec<u32>) {
    for &row in solution.iter() {
        for j in 0..solution.len() {
            if j == (row as usize) {
                print!("Q");
            } else {
                print!(".");
            }
        }
        println!("")
    }
}

fn main() {
    let solutions = get_queens_puzzle_soultions(4);
    for solution in solutions.iter() {
        pp_solution(solution);
        println!("");
    }
}
