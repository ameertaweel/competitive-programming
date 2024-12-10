use std::io::{self, BufRead};

const KEYWORD: &str = "MAS";

const DIAG_DIRS: [[(i64, i64); 2]; 2] = [
    [(1,  1), (-1, -1)],
    [(1, -1), (-1,  1)]
];

fn main() {
    if KEYWORD.len() % 2 != 1 {
        println!("ERROR: Keyword must have an odd number of characters.");
        return;
    }

    let stdin = io::stdin();

    // Read Grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect());
    }
    let grid = grid;

    let center_idx = KEYWORD.len() / 2;
    let center = KEYWORD.chars().nth(center_idx).unwrap();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in center_idx..(rows - center_idx) {
        'outer_1: for col in center_idx..(cols - center_idx) {
            if grid[row][col] != center { continue; }

            for diag_i_dirs in &DIAG_DIRS {
                let mut pass = false;

                'outer_2: for (x, y) in diag_i_dirs {
                    let x = *x;
                    let y = *y;

                    let mut row = (row as i64) + (center_idx as i64) * -y;
                    let mut col = (col as i64) + (center_idx as i64) * -x;

                    for i in 0..KEYWORD.len() {
                        let char_grid = grid[row as usize][col as usize];
                        let char_word = KEYWORD.chars().nth(i).unwrap();

                        row += y;
                        col += x;

                        if char_grid != char_word { continue 'outer_2; }
                    }

                    pass = true;
                }

                if !pass { continue 'outer_1; }
            }

            count += 1;
        }
    }

    println!("{}", count);
}
