// 26/08/2019
//
// Given a collection of intervals, merge all overlapping intervals. 
//
// Example:
// Input: [[1,3],[2,4],[6,9],[11,14]]
// Output: [[1,4],[6,9],[11,14]]
// Why?: Since intervals [1,3] and [2,4] overlaps, merge them into [1,4].
//
// Input: [[1,3],[3,4], [6,9]]
// Output: [[1,5], [6,9]]
// Why?: Intervals [1,3] and [3,4] are considered overlapping.

use std::cmp;
use std::fmt;

#[derive(Copy, Clone)]
struct Interval {
    start: u32,
    end: u32
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

fn merge(intervals: &Vec<Interval>) -> Vec<Interval> {
    if intervals.len() == 0 {
        return Vec::new();
    }

    let mut merged = vec![intervals[0]];
    for i in 1..intervals.len() {
        let n = merged.len() - 1;
        let mut last_interval = &mut merged[n];
        let interval = &intervals[i];

        if last_interval.end >= interval.start {
            last_interval.end = cmp::max(last_interval.end, interval.end);
        } else {
            merged.push(interval.clone());
        }
    }

    return merged;
}

fn main() {
    let input1 = vec![
        Interval{start: 1, end: 3},
        Interval{start: 2, end: 4},
        Interval{start: 6, end: 9},
        Interval{start: 11, end: 14}
    ]; 

    let output1 = merge(&input1);
    println!("Output1");
    for i in output1 {
        println!("{}", i);
    }

    let input2 = vec![
        Interval{start: 1, end: 3},
        Interval{start: 3, end: 4},
        Interval{start: 6, end: 9}
    ]; 

    let output2 = merge(&input2);
    println!("Output2");
    for i in output2 {
        println!("{}", i);
    }
}
