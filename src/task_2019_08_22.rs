// 22/08/2019
//
// Given a 2d grid map of '1's (land) and '0's (water), count the number of
// islands. An island is surrounded by water and is formed by connecting
// adjacent lands horizontally or vertically. You may assume all four edges of
// the grid are all surrounded by water. 
//
// Example:
//                      
//  Input:
//  01000
//  01000
//  00001
//  00000
//  Output: 2
//
//  Input:
//  11110
//  11010
//  11000
//  00000
//  Output: 1
//
//  Input:
//  11000
//  11000
//  00100
//  00011
//  Output: 3

fn is_visited(x: i32, y: i32, visited: &Vec<Vec<bool>>) -> bool {
    return visited[x as usize][y as usize];
}

fn is_land(x: i32, y: i32, map: &Vec<Vec<u8>>) -> bool {
    return map[x as usize][y as usize] == 1;
}

fn in_map(x: i32, y: i32, map: &Vec<Vec<u8>>) -> bool {
   return x >= 0 && y >= 0 && (x as usize) < map.len() && (y as usize) < map[0].len();
}

fn mark_visited(x: usize, y: usize, map: &Vec<Vec<u8>>, mut visited: &mut Vec<Vec<bool>>) {
    // println!("Mark visited X: {} Y: {}", x, y);
    visited[x][y] = true;
    for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let nx = (x as i32) + i;
        let ny = (y as i32) + j;
        if in_map(nx, ny, &map) && is_land(nx, ny, &map) && !is_visited(nx, ny, &visited) {
            mark_visited(nx as usize, ny as usize, &map, &mut visited);
        }
    }
}

fn count_islands(map: &Vec<Vec<u8>>) -> u32 {
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for row in map {
        let mut visited_row = Vec::new();
        for _ in row {
            visited_row.push(false);
        }
        visited.push(visited_row);
    }

    let mut num_of_islands = 0u32;
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if !visited[x][y] && map[x][y] == 1 {
                num_of_islands += 1;
                mark_visited(x, y, &map, &mut visited);
            }
        }
    } 

    return num_of_islands; 
}

fn main() {
    let map1 = vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0]
    ];
    let map2 = vec![
        vec![1, 1, 1, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0]
    ];
    let map3 = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 1]
    ];

    println!("Number of islands on map 1: {}", count_islands(&map1));
    println!("Number of islands on map 2: {}", count_islands(&map2));
    println!("Number of islands on map 3: {}", count_islands(&map3));
} 
