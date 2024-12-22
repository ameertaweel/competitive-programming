use std::io::{self, BufRead};

const DIRECTIONS: [(i64, i64); 4] = [
    (-1,  0),
    ( 0,  1),
    ( 1,  0),
    ( 0, -1)
];
const THRESHOLD: i64 = 100;

fn main() {
    let stdin = io::stdin();

    let (beg, end, is_wall) = {
        let mut beg = (0, 0);
        let mut end = (0, 0);
        let mut is_wall = Vec::new();
        for (row, line) in stdin.lock().lines().enumerate() {
            let line = line.unwrap();

            is_wall.push(vec![false; line.len()]);

            for (col, char) in line.chars().enumerate() {
                match char {
                    '#' => { is_wall[row][col] = true; },
                    'S' => { beg = (row, col); },
                    'E' => { end = (row, col); },
                     _  => { },
                }
            }
        }
        (beg, end, is_wall)
    };
    let rows = is_wall.len();
    let cols = is_wall[0].len();

    let mut pos = beg;
    let mut cost = 0;
    let mut cost_at = vec![vec![0; cols]; rows];

    while pos != end {
        for (row, col) in get_possible_moves(&pos, rows, cols) {
            if is_wall[row][col] { continue; }
            if cost_at[row][col] > 0 || (row, col) == beg { continue; }

            cost += 1;
            cost_at[row][col] = cost;
            pos = (row, col);
        }
    }

    let mut good_options = 0;
    for rowi in 0..rows {
        for coli in 0..cols {
            if is_wall[rowi][coli] { continue; }
            for (rowm, colm) in get_possible_moves(&(rowi, coli), rows, cols) {
                if !is_wall[rowm][colm] { continue; }
                let mut max_savings = -1;
                for (rowf, colf) in get_possible_moves(&(rowm, colm), rows, cols) {
                    if is_wall[rowf][colf] { continue; }
                    let savings = cost_at[rowf][colf] - cost_at[rowi][coli] - 2;
                    if savings > max_savings {
                        max_savings = savings;
                    }
                }
                if max_savings >= THRESHOLD {
                    good_options += 1;
                }
            }
        }
    }

    println!("{}", good_options);
}

fn within_grid(pos: &(i64, i64), rows: i64, cols: i64) -> bool {
    return pos.0 >= 0 && pos.0 < cols && pos.1 >= 0 && pos.1 < rows;
}

fn get_possible_moves(pos: &(usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let (row, col) = pos;
    let mut moves = vec![];
    for (delta_row, delta_col) in DIRECTIONS {
        let (next_row, next_col) = ((*row as i64) + delta_row, (*col as i64) + delta_col);
        if within_grid(&(next_row, next_col), rows as i64, cols as i64) {
            moves.push((next_row as usize, next_col as usize));
        }
    }
    return moves;
}
