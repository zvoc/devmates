// 05/09/2019
//
// Given a non-empty 2D array grid of 0's and 1's, an island is a group of 1's
// (representing land) connected 4-directionally (horizontal or vertical.) You
// may assume all four edges of the grid are surrounded by water.
//
// Find the maximum area of an island in the given 2D array. (If there is no
// island, the maximum area is 0.)
//
// Example:
//  Input:
//  [[0,0,1,0,0,0,0,1,0,0,0,0,0],
//   [0,0,0,0,0,0,0,1,1,1,0,0,0],
//   [0,1,1,0,1,0,0,0,0,0,0,0,0],
//   [0,1,0,0,1,1,0,0,1,0,1,0,0],
//   [0,1,0,0,1,1,0,0,1,1,1,0,0],
//   [0,0,0,0,0,0,0,0,0,0,1,0,0],
//   [0,0,0,0,0,0,0,1,1,1,0,0,0],
//   [0,0,0,0,0,0,0,1,1,0,0,0,0]]
//
//  Output: 6
//
//  Input: [[0,0,0,0,0,0,0,0]]
//  Output: 0
use std::cmp::max;

fn dfs(x: usize, y: usize, size: &mut u32,
       map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>> ) {
    *size = *size + 1;
    visited[x][y] = true;
    for &(i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
        // Check bounds
        if x == 0 && i == -1 || y == 0 && j == -1
            || x + 1 == map.len() && i == 1 || y + 1 == map[x].len() && j == 1 {
                continue;
        }

        let nx = ((x as i32) + i) as usize;
        let ny = ((y as i32) + j) as usize;

        // Check if land
        if map[nx][ny] == 0 {
            continue;
        }

        // Check if visited
        if visited[nx][ny] {
            continue;
        }

        dfs(nx, ny, size, map, visited);
    }
}

fn get_island_size(x: usize, y: usize,
                   map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>) -> u32 {
    let mut size = 0;
    dfs(x, y, &mut size, map, visited);
    return size;
}

fn get_max_island_size(map: &Vec<Vec<u8>>) -> u32 {
    let mut visited = Vec::new();
    for row in map.iter() {
        visited.push(vec![false; row.len()]);
    }

    let mut max_size = 0;
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == 1 && !visited[x][y] {
               max_size = max(max_size, get_island_size(x, y, map, &mut visited));
            }
        }
    }

    return max_size;
}

fn main() {

    let map = vec![
        vec![0,0,1,0,0,0,0,1,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
        vec![0,1,1,0,1,0,0,0,0,0,0,0,0],
        vec![0,1,0,0,1,1,0,0,1,0,1,0,0],
        vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,1,0,0],
        vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
        vec![0,0,0,0,0,0,0,1,1,0,0,0,0]
    ];
    println!("Max island size: {}", get_max_island_size(&map));

    let map = vec![
        vec![0,0,0,0,0,0,0,0]
    ];
    println!("Max island size: {}", get_max_island_size(&map));
}
