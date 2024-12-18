use std::io::{self, BufRead};
use std::collections::VecDeque;

const GRID_SIZE: i64 = 71;
const DIRECTIONS: [(i64, i64); 4] = [
    (-1,  0),
    ( 0,  1),
    ( 1,  0),
    ( 0, -1),
];

fn main() {
    let stdin = io::stdin();

    let mut bytes: Vec<(i64, i64)> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let coords: Vec<i64> = line.split(",").map(|c| c.parse().unwrap()).collect();
        bytes.push((coords[0], coords[1]));
    }
    let bytes = bytes;

    let mut start = 0;
    let mut end   = bytes.len() - 1;
    while (end - start) > 1 {
        let check = start + ((end - start) / 2);

        let mut is_corrupt: Vec<Vec<bool>> = vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize];
        for (row, col) in bytes.iter().take(check + 1) {
            is_corrupt[*row as usize][*col as usize] = true;
        }

        let mut stack: VecDeque<(i64, i64, i64)> = VecDeque::new();
        stack.push_back((0, 0, 0));

        let mut min_cost: Vec<Vec<i64>>;
        min_cost = vec![vec![GRID_SIZE * GRID_SIZE; GRID_SIZE as usize]; GRID_SIZE as usize];

        let mut path_found = false;

        while stack.len() > 0 {
            let (row, col, cost) = stack.pop_front().unwrap();

            if !within_grid(&(row, col), GRID_SIZE, GRID_SIZE) { continue; }
            if is_corrupt[row as usize][col as usize] { continue; }
            if min_cost[row as usize][col as usize] <= cost { continue; }

            min_cost[row as usize][col as usize] = cost;

            if row == GRID_SIZE - 1 && col == GRID_SIZE - 1 {
                path_found = true;
                continue;
            }

            for (drow, dcol) in DIRECTIONS {
                stack.push_back((row + drow, col + dcol, cost + 1));
            }
        }

        if path_found {
            start = check;
        } else {
            end = check;
        }
    }

    let (row, col) = bytes[end];
    println!("{},{}", row, col);
}

fn within_grid(pos: &(i64, i64), rows: i64, cols: i64) -> bool {
    return pos.0 >= 0 && pos.0 < cols && pos.1 >= 0 && pos.1 < rows;
}
