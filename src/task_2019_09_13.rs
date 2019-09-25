// 13/09/2019
//
// A gene string can be represented by an 8-character long string, with
// choices from "A", "C", "G", "T".
//
// Suppose we need to investigate about a mutation (mutation from "start" to
// "end"), where ONE mutation is defined as ONE single character changed in
// the gene string.
//
// For example, "AACCGGTT" -> "AACCGGTA" is 1 mutation.
//
// Also, there is a given gene "bank", which records all the valid gene
// mutations. A gene must be in the bank to make it a valid gene string.
//
// Now, given 3 things - start, end, bank, your task is to determine what is
// the minimum number of mutations needed to mutate from "start" to "end". If
// there is no such a mutation, return -1.
//
// Note
//  - Starting point is assumed to be valid, so it might not be included in the bank.
//  - If multiple mutations are needed, all mutations during in the sequence must be valid.
//  - You may assume start and end string is not the same.
//
// Example:
//  start: "AACCGGTT"
//  end:   "AAACGGTA"
//  bank: ["AACCGGTA", "AACCGCTA", "AAACGGTA"]
//  return: 2
//
//  start: "AAAAACCC"
//  end:   "AACCCCCC"
//  bank: ["AAAACCCC", "AAACCCCC", "AACCCCCC"]
//  return: 3

use std::cmp::{min, max};
use std::collections::HashSet;

fn get_diff(s1: &str, s2: &str) -> usize {
    // To handle string of different length
    let mut diff = max(s1.len(), s2.len()) - min(s1.len(), s2.len());
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
    }
    diff
}

fn dfs(len: usize, best_len: &mut Option<usize>, start: usize, end: usize,
       visited: &mut HashSet<usize>, bank: &Vec<&str>) {
    visited.insert(start);

    if get_diff(bank.get(start).unwrap(), bank.get(end).unwrap()) == 1 {
        *best_len = Some(best_len.map_or_else(|| len, |l| min(l, len)));
    }

    for i in 0..bank.len() {
        if visited.contains(&i) { continue; }

        if get_diff(bank.get(i).unwrap(), bank.get(start).unwrap()) == 1 {
            dfs(len + 1, best_len, i, end, visited, bank);
        }
    }
}

fn find_mutation_path_len<'a>(start: &'a str, bank: &mut Vec<&'a str>,
                          end: &'a str) -> Option<usize> {
    bank.push(start);
    bank.push(end);

    let mut best_len = None;
    let mut visited = HashSet::new();

    dfs(1, &mut best_len, bank.len() - 2, bank.len() - 1, &mut visited, &bank);

    bank.pop();
    bank.pop();

    best_len
}

fn main() {
    let start = "AACCGGTT";
    let end = "AAACGGTA";
    let mut bank = vec!["AACCGGTA", "AACCGCTA", "AAACGGTA"];
    if let Some(distance) = find_mutation_path_len(&start, &mut bank, &end) {
        println!("Min distance: {}", distance);
    } else {
        println!("No path");
    }

    let start = "AAAAACCC";
    let end = "AACCCCCC";
    let mut bank = vec!["AAAACCCC", "AAACCCCC", "AACCCCCC"];
    if let Some(distance) = find_mutation_path_len(&start, &mut bank, &end) {
        println!("Min distance: {}", distance);
    } else {
        println!("No path");
    }
}
