use std::io::{self, BufRead};

const DIRECTIONS: [(i64, i64); 4] = [
    (-1,  0),
    ( 0,  1),
    ( 1,  0),
    ( 0, -1)
];
const MAX_CHEAT: usize = 20;
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
            let rowf_i = if rowi > MAX_CHEAT { rowi - MAX_CHEAT } else { 0 };
            let colf_i = if coli > MAX_CHEAT { coli - MAX_CHEAT } else { 0 };
            let rowf_f = if rowi + MAX_CHEAT + 1 < rows { rowi + MAX_CHEAT + 1 } else { rows };
            let colf_f = if coli + MAX_CHEAT + 1 < cols { coli + MAX_CHEAT + 1 } else { cols };
            for rowf in rowf_i..rowf_f {
                for colf in colf_i..colf_f {
                    if is_wall[rowf][colf] { continue; }

                    let cheat_cost = rowf.abs_diff(rowi) + colf.abs_diff(coli);
                    if cheat_cost > MAX_CHEAT { continue; }

                    let savings = (cost_at[rowf][colf] as i64) - (cost_at[rowi][coli] as i64) - (cheat_cost as i64);
                    if savings >= THRESHOLD {
                        good_options += 1;
                    }
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
