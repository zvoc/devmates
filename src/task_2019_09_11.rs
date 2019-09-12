// 11/09/2019
//
// In a warehouse, there is a row of barcodes, where the i-th barcode is
// barcodes[i].
//
// Rearrange the barcodes so that no two adjacent barcodes are equal. You may
// return any answer, and it is guaranteed an answer exists.
//
// Example:
//  Input: [1,1,1,2,2,2]
//  Output: [2,1,2,1,2,1]
//
//  Input: [1,1,1,1,2,2,3,3]
//  Output: [1,3,1,3,2,1,2,1]

use std::collections::HashMap;
use std::mem::drop;

fn dfs(target_len: usize, solution: &mut Vec<u32>,
       barcode_count: &mut Vec<(u32, u32)>) -> bool {
    if solution.len() == target_len {
       return true
    }

    for i in 0..barcode_count.len() {
        let (barcode, ref mut count) = barcode_count.get_mut(i).unwrap();
        if solution.len() > 0 && solution[solution.len() - 1] == *barcode {
            continue;
        }

        if *count == 0 {
            continue;
        }

        solution.push(*barcode);
        *count -= 1;
        drop(barcode);
        drop(count);

        if dfs(target_len, solution, barcode_count) {
            return true;
        }

        let (_, ref mut count) = barcode_count.get_mut(i).unwrap();
        *count += 1;
        solution.pop();
    }

    return false;
}

fn rearange(barcodes: &Vec<u32>) -> Option<Vec<u32>> {
    let mut barcode_count = HashMap::new();
    for &barcode in barcodes.iter() {
        *barcode_count.entry(barcode).or_insert(0u32) += 1;
    }

    let mut barcode_count: Vec<(u32, u32)> = barcode_count.into_iter().collect();
    let mut solution = Vec::new();
    if dfs(barcodes.len(), &mut solution, &mut barcode_count) {
        Some(solution)
    } else {
        None
    }
}

fn main() {
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    if let Some(ref aranged) = rearange(&barcodes) {
        for &barcode in aranged.iter() {
            print!("{} ", barcode);
        }
        println!("");
    } else {
        println!("No solution");
    }

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    if let Some(ref aranged) = rearange(&barcodes) {
        for &barcode in aranged.iter() {
            print!("{} ", barcode);
        }
        println!("");
    } else {
        println!("No solution");
    }
}
