// 23/08/2019
//
// You are given coins of different denominations and a total amount of money
// amount. Write a function to compute the fewest number of coins that you
// need to make up that amount. You may assume that you have an infinite
// number of each kind of coin. If that amount of money cannot be made up by
// any combination of the coins, return -1. 
//
// Example:
// Input: coins = [1, 2, 5], amount = 13
// Output: 4
// Why?: 11 = 5 + 5 + 2 + 1
//
// Input: coins = [5, 10], amount = 14
// Output: -1

use std::cmp;

fn dfs(target: u32, coins: &Vec<u32>, mut amount: u32,
       depth: u32, mut min_num_of_coins: Option<u32>) -> Option<u32> {
    if amount > target {
        return min_num_of_coins;
    }

    if amount == target {
        min_num_of_coins = if let Some(c) = min_num_of_coins {
            Some(cmp::min(c, depth))
        } else {  
            Some(depth)
        }
    }

    for coin in coins.iter() {
        amount += coin;
        min_num_of_coins = dfs(target, coins, amount, depth + 1, min_num_of_coins);
        amount -= coin;
    }

    return min_num_of_coins;
}

fn get_num_of_coins(target: u32, coins: &Vec<u32>) -> Option<u32> {
    return dfs(target, &coins, 0u32, 0, None);
}

fn main() {
    let coins1 = vec![1, 2, 5];
    let target1 = 13;
    if let Some(num_of_coins) = get_num_of_coins(target1, &coins1) {
        println!("Number of coins: {}", num_of_coins);
    } else {
        println!("No such number");
    }

    let coins2 = vec![5, 10];
    let target2 = 14;
    if let Some(num_of_coins) = get_num_of_coins(target2, &coins2) {
      println!("Number of coins: {}", num_of_coins);
    } else {
      println!("No such number");
    }
}
