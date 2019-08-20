// 19/08/2019
//
// Given a binary tree, imagine yourself standing on the right side of it,
// return the values of the nodes you can see ordered from top to bottom. 
//
// Example:
// Input: [1,2,3,null,5,null,4]
// Output: [1, 3, 4]
// Explanation:
// 
//      1            <---
//    /   \
//   2     3         <---
//    \     \
//     5     4       <---

fn _get_right_view(depth: usize, index: usize,
                   tree: &Vec<Option<i32>>, result: &mut Vec<i32>) {
    if result.len() == depth && index <= tree.len() {
        if let Some(x) = tree[index - 1] {
            result.push(x);
       }
    }

    let right_index = 2 * index + 1;
    if right_index <= tree.len() {
        _get_right_view(depth + 1, right_index, tree, result);
    }

    let left_index = 2 * index;
    if left_index <= tree.len() {
        _get_right_view(depth + 1, left_index, tree, result);
    }
}

fn get_right_view(tree: &Vec<Option<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    _get_right_view(0, 1, tree, &mut result);
    return result;
}


fn main() {
    let tree = vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)];
    let result = get_right_view(&tree); 

    println!("Result:");
    for r in result {
        println!("{}", r);
    }
}
