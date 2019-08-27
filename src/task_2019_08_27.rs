// 27/08/2019
//
// Given a collection of candidate numbers and a target number, find all
// unique combinations in candidates where the candidate numbers sums to
// target. Each number in candidates may only be used once in the combination.
// All numbers will be positive integers. The solution set must not contain
// duplicate combinations.
//
// Example:
// Input: candidates = [10,1,2,7,6,1,5], target = 8
// Output:
//   [
//     [1, 7],
//     [1, 2, 5],
//     [2, 6],
//     [1, 1, 6]
//   ]
//
// Input: candidates = [2,5,2,1,2], target = 5,
// Output:
//   [
//     [1,2,2],
//     [5]
//   ]

use std::collections::HashMap;

fn dfs(depth: usize, sum: u32, target: u32, count: &mut Vec<u32>,
       solutions: &mut Vec<Vec<u32>>, candidates: &Vec<(u32, u32)>) {
    if sum == target {
        let mut solution = Vec::new();
        for i in 0..count.len() {
            let (val, _) = candidates[i];
            for _ in 0..count[i] {
                solution.push(val);
            }
        }

        solutions.push(solution);
        return;
    }

    if depth < count.len() {
        let (val, max_count) = candidates[depth];
        for i in 0..=max_count {
            count[depth] = i;
            dfs(depth + 1, sum + val * i, target, count, solutions, candidates);
        }
        count[depth] = 0;
    }
}

fn find_soulutions(candidates: &Vec<u32>, target: u32) -> Vec<Vec<u32>> {
    let mut candidates_count_map: HashMap<u32, u32> = HashMap::new();
    for &candidate in candidates {
        let val = if candidates_count_map.contains_key(&candidate) {
            candidates_count_map[&candidate] + 1
        } else {
            1
        };
        candidates_count_map.insert(candidate, val);
    }

    let mut candidates_count = Vec::new();
    for i in candidates_count_map {
        candidates_count.push(i);
    }

    let mut solutions = Vec::new();
    let mut count = vec![0; candidates_count.len()];
    dfs(0, 0, target, &mut count, &mut solutions, &candidates_count);
    return solutions;
}

fn main() {
    let candidates1 = vec![10, 1, 2, 7, 6, 1, 5];
    let target1 = 8;
    let solutions1 = find_soulutions(&candidates1, target1);
    println!("Solutions 1");
    println!("[");
    for ss in solutions1 {
        print!("  [ ");
        for s in ss {
            print!("{} ", s);
        }
        println!("]");
    }
    println!("]");

    let candidates2 = vec![2, 5, 2, 1, 2];
    let target2 = 5;
    let solutions2 = find_soulutions(&candidates2, target2);
    println!("Solutions 2");
    println!("[");
    for ss in solutions2 {
        print!("  [ ");
        for s in ss {
            print!("{} ", s);
        }
        println!("]");
    }
    println!("]");
}
