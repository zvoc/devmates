// 02/08/2019
//
// Given an array of integers nums and an integer k, you need to find the total
// number of continuous subarrays whose sum equals to k.
//
// Example:
// Input: nums = [1,2,1,2,1], k = 3
// Output: 4
//
// Input: nums = [1,1,1], k = 2
// Output: 2

fn count_subarrays(target: u32, list: &Vec<u32>) -> u32 {
    let mut i = 0; // Inclusive
    let mut j = 0; // Exclusive
    let mut sum = 0; // Sum of [i, j)
    let mut count = 0;
    while j < list.len() {
        while j < list.len() && sum < target {
            sum += list[j];
            j += 1;
        }

        // Sum is either == target or bigger
        if sum == target {
            count += 1;
        }

        while sum >= target {
            sum -= list[i];
            i += 1;
        }
    }

    return count;
}

fn main() {
    let nums = vec![1, 2, 1, 2, 1];
    println!("Subarrays: {}", count_subarrays(3, &nums));
    let nums = vec![1, 1, 1];
    println!("Subarrays: {}", count_subarrays(2, &nums));
}
