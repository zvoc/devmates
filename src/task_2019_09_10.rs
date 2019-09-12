// 10/09/2019
//
// An image is represented by a 2-D Array of integers, each integer
// representing the pixel value of the image (from 0 to 65535).
//
// Given a coordinate (sr, sc) representing the starting pixel (row and
// column) of the flood fill, and a pixel value newColor, "flood fill" the
// image.
//
// To perform a "flood fill", consider the starting pixel, plus any pixels
// connected 4-directionally to the starting pixel of the same color as the
// starting pixel, plus any pixels connected 4-directionally to those pixels
// (also with the same color as the starting pixel), and so on. Replace the
// color of all of the aforementioned pixels with the newColor.
//
// At the end, return the modified image.
//
// Example:
//  Input:
//    image =  [[1,1,1],
//              [1,1,0],
//              [1,0,1]]
//    sr = 1, sc = 1, newColor = 2
//
//  Output:     [[2,2,2],
//               [2,2,0],
//               [2,0,1]]

use std::result::Result;

fn dfs(x: usize, y: usize, old_color: u16, new_color: u16, img: &mut Vec<Vec<u16>>) {
    if img[x][y] == new_color {
        return;
    }

    img[x][y] = new_color;
    for &(i, j) in [(0, -1), (0, 1), (1, 0), (-1, 0)].iter() {
        // Check img bounds
        if x == 0 && i == -1
            || y == 0 && j == -1
            || x + 1 == img.len() && i == 1
            || y + 1 == img[x].len() && j == 1 {
            continue;
        }

        let nx = ((x as i32) + i) as usize;
        let ny = ((y as i32) + j) as usize;

        // Check color
        if img[nx][ny] != old_color {
            continue;
        }

        dfs(nx, ny, old_color, new_color, img);
    }
}

fn flood_fill(x: usize, y: usize, new_color: u16,
              img: &mut Vec<Vec<u16>>) -> Result<(), &str> {
    if x >= img.len() || y >= img[0].len() {
        Err("Invalid starting point")
    } else {
        dfs(x, y, img[x][y], new_color, img);
        Ok(())
    }
}

fn print_image(image: &Vec<Vec<u16>>) {
    for row in image.iter() {
        for pixel in row.iter() {
            print!("{}", pixel);
        }
        println!("");
    }
}

fn main() {
    let mut image =  vec![
        vec![1,1,1],
        vec![1,1,0],
        vec![1,0,1]
    ];

    println!("Before");
    print_image(&image);

    flood_fill(1, 1, 2, &mut image);

    println!("After");
    print_image(&image);
}
