use std::io::{self, BufRead};

const KEYWORD: &str = "XMAS";
const DIRS: [(i64, i64); 8] = [
    (0,  1), ( 1,  1), ( 1, 0), ( 1, -1),
    (0, -1), (-1, -1), (-1, 0), (-1,  1)
];

fn main() {
    let stdin = io::stdin();

    // Read Grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect());
    }
    let grid = grid;

    let initial = KEYWORD.chars().nth(0).unwrap();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != initial { continue; }

            'outer: for (x, y) in &DIRS {
                let x = *x;
                let y = *y;

                if x ==  1 && cols - col < KEYWORD.len() { continue; }
                if x == -1 &&    1 + col < KEYWORD.len() { continue; }
                if y ==  1 && rows - row < KEYWORD.len() { continue; }
                if y == -1 &&    1 + row < KEYWORD.len() { continue; }

                let mut row = row as i64;
                let mut col = col as i64;
                for i in 0..KEYWORD.len() {
                    let char_grid = grid[row as usize][col as usize];
                    let char_word = KEYWORD.chars().nth(i).unwrap();

                    row += y;
                    col += x;

                    if char_grid != char_word { continue 'outer; }
                }

                count += 1;
            }
        }
    }

    let mut is_palindrome = true;
    let mut i = 0;
    let mut j = KEYWORD.len() - 1;
    while i < j {
        let char_i = KEYWORD.chars().nth(i).unwrap();
        let char_j = KEYWORD.chars().nth(j).unwrap();

        if char_i != char_j {
            is_palindrome = false;
            break;
        }

        i += 1;
        j -= 1;
    }

    if is_palindrome {
        count = count / 2;
    }

    println!("{}", count);
}
