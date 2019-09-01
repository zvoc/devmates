// 28/08/2019
//
// Write an efficient algorithm that searches for a value in an M x N matrix.
// This matrix has the following properties:
//
//  Integers in each row are sorted in ascending from left to right.
//  Integers in each column are sorted in ascending from top to bottom.
//
// Example:
// Matrix = [
//            [1,   4,  7, 11, 15],
//            [2,   5,  8, 12, 19],
//            [3,   6,  9, 16, 22],
//            [10, 13, 14, 17, 24],
//            [18, 21, 23, 26, 30]
//          ]
//  Input: target = 8
//  Output:  true
//  Input: target = 29
//  Output: false

fn search(target: i32, x1: i32, x2: i32,
          y1: i32, y2: i32, matrix: &Vec<Vec<i32>>) -> bool {
    // Stopping condition
    if x1 > x2 || y1 > y2 || x1 < 0 || y1 < 0
        || (x2 as usize) >= matrix.len() 
        || (y2 as usize) >= matrix[0].len() {
        return false;
    }

    let x_mid = x1 + (x2 - x1) / 2;
    let y_mid = y1 + (y2 - y1) / 2;
    let current = matrix[x_mid as usize][y_mid as usize];

    if current == target {
        return true;

    } else if current < target {
        // I can eliminate upper left part of matrix
        return search(target, x_mid + 1, x2, y1, y_mid, matrix)
            || search(target, x1, x2, y_mid + 1, y2, matrix);
        
    } else {
        // I can eliminate bottom right part of matrix
        return search(target, x1, x2, y1, y_mid - 1, matrix)
            || search(target, x1, x_mid -1, y_mid, y2, matrix);
    }
}

fn search_matrix(target: i32, matrix: &Vec<Vec<i32>>) -> bool {
    return search(target, 0, (matrix.len() -1) as i32,
                  0, (matrix[0].len() - 1) as i32, matrix);
}

fn main() {
    let matrix = vec![
      vec![1,   4,  7, 11, 15],
      vec![2,   5,  8, 12, 19],
      vec![3,   6,  9, 16, 22],
      vec![10, 13, 14, 17, 24],
      vec![18, 21, 23, 26, 30]
    ];

    println!("Is present: {}", search_matrix(8, &matrix));
    println!("Is present: {}", search_matrix(29, &matrix));
}
