// 16/09/2019
//
// Given a string, find the length of the longest substring without repeating
// characters.
//
// Example:
//  Input: "adddw"
//  Output: "2"
//
//  Input: "abcdabcbb"
//  Output: 4
//
//  Input: "ccccccc"
//  Output: "1"
use std::collections::HashSet;
use std::cmp::max;

fn get_longest_nonrep_substr_len(string: &str) -> usize {
    let letters = string.as_bytes();
    let mut curr_letters = HashSet::new();
    let mut max_len = 0;
    let mut i = 0;
    let mut j = 0;

    while i < letters.len() && j < letters.len() {
        while i < letters.len() && !curr_letters.contains(&letters[i]) {
            curr_letters.insert(&letters[i]);
            i += 1;
        }

        max_len = max(max_len, curr_letters.len());

        while j < letters.len() && curr_letters.contains(&letters[j]) {
            curr_letters.remove(&letters[j]);
            j += 1;
        }
    }

    max_len
}

fn main() {
    let input = "adddw";
    println!("Output: {}", get_longest_nonrep_substr_len(&input));
    let input = "abcdabcbb";
    println!("Output: {}", get_longest_nonrep_substr_len(&input));
    let input = "ccccccc";
    println!("Output: {}", get_longest_nonrep_substr_len(&input));
}
