// 19/09/2019
//
// Given N pairs of parentheses, write a function to generate all combinations
// of well-formed parentheses.
//
// Example:
//  Input: N = 3
//  Output:
//    [
//      "((()))",
//      "(()())",
//      "(())()",
//      "()(())",
//      "()()()"
//    ]

fn dfs(left_remaining: &mut u32, right_remaining: &mut u32,
       level: &mut u32, string: &mut String, parentheses: &mut Vec<String>) {
    if *left_remaining == 0 && *right_remaining == 0 {
        parentheses.push(string.clone());
        return;
    }

    if *left_remaining > 0 {
        *left_remaining -= 1;
        *level += 1;
        string.push('(');
        dfs(left_remaining, right_remaining, level, string, parentheses);
        string.pop();
        *level -= 1;
        *left_remaining += 1;
    }

    if *right_remaining > 0 && *level > 0 {
        *right_remaining -= 1;
        *level -= 1;
        string.push(')');
        dfs(left_remaining, right_remaining, level, string, parentheses);
        string.pop();
        *level += 1;
        *right_remaining += 1;
    }
}

fn get_all_parentheses(n: u32) -> Vec<String> {
    let mut parentheses = Vec::new();
    if n > 0 {
        let mut left_remaining = n;
        let mut right_remaining = n;
        let mut level = 0;
        let mut string = String::new();
        dfs(&mut left_remaining, &mut right_remaining,
            &mut level, &mut string, &mut parentheses);
    }
    parentheses
}

fn main() {
    let n = 3;
    let parentheses = get_all_parentheses(n);
    for p in parentheses.iter() {
        println!("{}", *p);
    }

}
