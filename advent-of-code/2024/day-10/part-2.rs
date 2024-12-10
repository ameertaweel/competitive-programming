use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }
    let grid = grid;
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;

    let mut score = 0;
    for row in 0..rows {
        for col in 0..cols {
            let height = grid[row as usize][col as usize];
            if height != 0 { continue; }

            let mut stack: Vec<(i64, i64, u8)> = Vec::new();
            stack.push((row as i64, col as i64, height));

            let mut trailhead_score = 0;

            while stack.len() > 0 {
                let (row, col, height) = stack.pop().unwrap();
                
                if height == 9 {
                    trailhead_score += 1;
                    continue;
                }

                let possible_next: Vec<(i64, i64)> = vec![
                    (row + 1, col),
                    (row - 1, col),
                    (row, col + 1),
                    (row, col - 1),
                ];

                for (row, col) in possible_next {
                    if row < 0 || row >= rows || col < 0 || col >= cols { continue; }
                    let next_height = grid[row as usize][col as usize];

                    if next_height == height + 1 {
                        stack.push((row, col, next_height));
                    }
                }
            }

            score += trailhead_score;
        }
    }

    println!("{}", score);
}
