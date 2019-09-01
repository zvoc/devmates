// 29/08/2019
//
// In a given 2D binary array A, there are two islands. (An island is a
// 4-directionally connected group of 1s not connected to any other 1s.) Now,
// we may change 0s to 1s so as to connect the two islands together to form 1
// island. Return the smallest number of 0s that must be flipped. (It is
// guaranteed that the answer is at least 1.) 
//
// Example:
//  Input:
//    A = [
//          [0,1],
//          [1,0]
//        ]
//
//  Output:  1
//  Why?: 
//    A = [
//          [1,1],
//          [1,0]
//        ]
//
//  Input: 
//    A = [
//          [0,1,0],
//          [0,0,0],
//          [0,0,1]
//        ]
//  Output: 2
//  Why?:
//    A = [
//          [0,1,1],
//          [0,0,1],
//          [0,0,1]
//        ]
//
//  Input: 
//    A = [
//          [1,1,1,1,1],
//          [1,0,0,0,1],
//          [1,0,1,0,1],
//          [1,0,0,0,1],
//          [1,1,1,1,1]]
//  Output: 1
//  Why?:
//    A = [
//          [1,1,1,1,1],
//          [1,0,1,0,1],
//          [1,0,1,0,1],
//          [1,0,0,0,1],
//          [1,1,1,1,1]]

use std::collections::VecDeque;
use std::collections::HashSet;

fn find_starting_point(map: &Vec<Vec<u8>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 1 {
                return (i, j);
            }
        }
    }

    panic!("No islands");
}

fn get_visited(map: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut visited = Vec::new();
    for row in map {
       visited.push(vec![false; row.len()]);
    }
    return visited;
}

fn dfs(point: (usize, usize), map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>,
       coastline: &mut HashSet<(usize, usize)>) {
    let (x, y) = point;
    if visited[x][y] {
        return;
    }

    if map[x][y] == 0 {
        coastline.insert(point);
        return;
    }

    visited[x][y] = true;
    for &(i, j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        if x == 0 && i == -1 || y == 0 && j == -1 
            || x + 1 == map.len() && i == 1 || y + 1== map[0].len() && j == 1 {
            continue;
        }

        let nx = ((x as i32) + i) as usize;
        let ny = ((y as i32) + j) as usize;

        dfs((nx, ny), map, visited, coastline);
    }
}

fn get_coastline(start_point: (usize, usize), map: &Vec<Vec<u8>>,
                   visited: &mut Vec<Vec<bool>>) -> HashSet<(usize, usize)> {
    let mut coastline = HashSet::new();
    dfs(start_point, map, visited, &mut coastline);
    return coastline;
}

fn connect_islands(map: &Vec<Vec<u8>>) -> u32 {
    let mut visited = get_visited(map);
    let coastline = get_coastline(find_starting_point(map), map, &mut visited);
    let mut q = VecDeque::new();
    for c in coastline {
        q.push_back((c, 0));
    }

    while !q.is_empty() {
        let ((x, y), d) = q.pop_front().unwrap();
        if visited[x][y] {
            continue;
        }
        
        visited[x][y] = true;
        if map[x][y] == 1 {
            return d;
        }

        
        for &(i, j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            if x == 0 && i == -1 || y == 0 && j == -1 
                || x + 1 == map.len() && i == 1 || y + 1 == map[0].len() && j == 1 {
                continue;
            }

            let nx = ((x as i32) + i) as usize;
            let ny = ((y as i32) + j) as usize;
            
            if visited[nx][ny] {
                continue;
            }
            
            q.push_back(((nx, ny), d + 1));
        }
    }

    panic!("This should not execute");
}

fn main() {
    let map1 = vec![
      vec![0,1],
      vec![1,0]
    ];
    
    println!("Output {}", connect_islands(&map1));

    let map2 = vec![
      vec![0,1,0],
      vec![0,0,0],
      vec![0,0,1]
    ];
    println!("Output {}", connect_islands(&map2));

    let map3 = vec![
      vec![1,1,1,1,1],
      vec![1,0,0,0,1],
      vec![1,0,1,0,1],
      vec![1,0,0,0,1],
      vec![1,1,1,1,1]
    ];
    println!("Output {}", connect_islands(&map3));
}
