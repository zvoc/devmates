// 20/08/2019
//
// Suppose an array sorted in ascending order is rotated at some pivot unknown
// to you beforehand. You are given a target value to search. If found in the
// array return its index, otherwise return -1. You may assume no duplicate
// exists in the array. Solution should be faster than O(n). 
//
// Example:
// Input: nums = [5,7,9,0,1,2,3], target = 0
// Output: 3
//
// Input: nums = [5,7,9,0,1,2,3], target = 8
// Output: -1

fn find(list: &Vec<i32>, target: i32) -> Option<usize> {
    if list.len() == 0 {
        return None;
    }

    let mut left = 0;
    let mut right = list.len() - 1;

    while right - left > 1 { 
        let probe = left + (right - left) / 2;
        if list[left] < list[probe] {
            // Left index - probe is sorted
            if list[left] < target && target < list[probe] {
                right = probe; 
            } else {
                left = probe + 1;
            }

        } else {
            // Probe - right index is sorted
            if list[probe] < target && target < list[right] {
                left = probe;
            } else {
                right = probe - 1;
            }
        }
    }

    if list[left] == target {
        return Some(left);
    } else if list[right] == target {
        return Some(right);
    } else {
        return None;
    }
}

fn main() {
  let list: Vec<i32> = vec![5, 7, 9, 0, 1, 2, 3];
  let target = 2;
  let index = find(&list, target);
  if let Some(i) = index {
    println!("Index: {}", i);
  } else {
    println!("Can't find target");  
  }
}
