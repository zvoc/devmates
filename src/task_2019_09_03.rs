// 03/09/2019
//
// A message containing letters from A-Z is being encoded to numbers using the
// following mapping:
//
//  'A' -> 1
//  'B' -> 2
//  ...
//  'Z' -> 26
//
// Given a non-empty string containing only digits, determine the total number
// of ways to decode it. 
//
// Example:
//  Input: "12"
//  Output: 2
//  Why? "AB"(1 2), "L"(12)
//
//  Input: "226"
//  Output: 3
//  Why? "BZ"(2 26) "VF"(22 6) "BBF"(2 2 6)

fn count_combinations(string: &str) -> u32 {
    if string.len() <= 1 {
        return 1;
    }

    let mut comb = vec![1, 1];
    for (c0, c1) in string.chars().zip(string.chars().skip(1)) {
        let n = comb.len();
        if c0 == '1' || (c0 == '2' && c1 <= '6') {
           comb.push(comb[n - 2] + comb[n - 1]);
        } else {
            comb.push(comb[n - 1]);
        }

    }

    return comb[comb.len() - 1];

}

fn main() {
    let input = "12";
    println!("{}", count_combinations(input));
    let input = "226";
    println!("{}", count_combinations(input));
}
