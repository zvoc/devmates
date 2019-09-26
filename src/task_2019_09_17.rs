// 17/09/2019
//
// Given a char array representing tasks CPU need to do. It contains capital
// letters A to Z where different letters represent different tasks. Tasks
// could be done without original order. Each task could be done in one
// interval. For each interval, CPU could finish one task or just be idle.
//
// However, there is a non-negative cooling interval n that means between two
// same tasks, there must be at least n intervals that CPU are doing different
// tasks or just be idle.
//
// You need to return the least number of intervals the CPU will take to
// finish all the given tasks.
//
// Example:
//  Input: tasks = ["A","A","A","B","B","B"], n = 2
//  Output: 8
//  Why? A->B->idle->A->B->idle->A->B.

use std::collections::HashMap;
use std::u32::MAX;
use std::cmp::min;

struct TaskCount {
    max: u32,
    stack: Vec<u32>
}

impl TaskCount {
    fn new() -> Self {
        TaskCount{max: 1, stack: Vec::new()}
    }
}

fn dfs(min_intervals: &mut u32, tasks_remaining: u32, cooldown: u32,
       depth: u32, tasks: &Vec<&str>, counts: &mut Vec<TaskCount>) {
    if tasks_remaining == 0 {
        *min_intervals = min(*min_intervals, depth);
        return;
    }

    let mut found_option = false;
    for (i, task) in tasks.iter().enumerate() {
        {
            let mut c = counts.get_mut(i).unwrap();
            if c.stack.len() == (c.max as usize) {
                continue;
            }

            let len = c.stack.len();
            if len > 0 && c.stack.get(len - 1).unwrap() + cooldown >= depth {
                continue;
            }

            c.stack.push(depth);
            found_option = true;
        }
        dfs(min_intervals, tasks_remaining - 1, cooldown, depth + 1, tasks, counts);
        {
            let mut c = counts.get_mut(i).unwrap();
            c.stack.pop();
        }
    }
    if !found_option {
        dfs(min_intervals, tasks_remaining, cooldown, depth + 1, tasks, counts);
    }
}

fn get_cpu_intervals(tasks: &Vec<&str>, cooldown: u32) -> u32 {
    let mut tasks_count: HashMap<&str, TaskCount> = HashMap::new();
    for task in tasks {
        if let Some(c) = tasks_count.get_mut(task) {
            (*c).max += 1;
        } else {
            tasks_count.insert(task, TaskCount::new());
        }
    }

    let mut tasks = Vec::new();
    let mut counts = Vec::new();
    let mut num_of_tasks = 0;
    for (k, v) in tasks_count.drain() {
        num_of_tasks += v.max;
        tasks.push(k);
        counts.push(v);
    }

    let mut min_intervals = MAX;
    dfs(&mut min_intervals, num_of_tasks, cooldown, 0, &tasks, &mut counts);
    min_intervals
}

fn main() {
    let tasks = vec!["A","A","A","B","B","B"];
    let cooldown = 2;
    println!("Cpu counts: {}", get_cpu_intervals(&tasks, cooldown));
}
