use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn main() {
    let stdin = io::stdin();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect());
    }
    let grid = grid;
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;

    // Row, Col
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    // (Origin Row, Origin Col), (Area, Corners)
    let mut region: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    // (Origin Row, Origin Col), (Row, Col)
    let mut stack: Vec<(i64, i64, i64, i64)> = Vec::new();
    
    for row in 0..rows {
        for col in 0..cols {
            stack.push((row, col, row, col));
        }
    }

    while stack.len() > 0 {
        let (origin_row, origin_col, row, col) = stack.pop().unwrap();

        if seen.contains(&(row, col)) { continue; }

        let plant = grid[row as usize][col as usize];

        let (area, corners) = region.get(&(origin_row, origin_col)).unwrap_or(&(0, 0));
        let mut delta_corners = 0;

        let possible_next: Vec<(i64, i64)> = vec![
            (row + 1, col),
            (row, col + 1),
            (row - 1, col),
            (row, col - 1),
        ];

        for (row, col) in possible_next {
            if row < 0 || row >= rows || col < 0 || col >= cols { continue; }

            if grid[row as usize][col as usize] != plant { continue; }

            if seen.contains(&(row, col)) { continue; }

            stack.push((origin_row, origin_col, row, col));
        }

        let possible_corners: Vec<_> = vec![
            (row + 1, col    , row    , col + 1, row + 1, col + 1),
            (row    , col + 1, row - 1, col    , row - 1, col + 1),
            (row - 1, col    , row    , col - 1, row - 1, col - 1),
            (row    , col - 1, row + 1, col    , row + 1, col - 1),
        ];

        for (row1, col1, row2, col2, row3, col3) in possible_corners {
            let empty1 = row1 < 0 || row1 >= rows || col1 < 0 || col1 >= cols || grid[row1 as usize][col1 as usize] != plant;
            let empty2 = row2 < 0 || row2 >= rows || col2 < 0 || col2 >= cols || grid[row2 as usize][col2 as usize] != plant;

            if empty1 && empty2 {
                delta_corners += 1;
                continue;
            }

            let empty3 = row3 < 0 || row3 >= rows || col3 < 0 || col3 >= cols || grid[row3 as usize][col3 as usize] != plant;

            if !empty1 && !empty2 && empty3 {
                delta_corners += 1;
            }
        }

        region.insert((origin_row, origin_col), (area + 1, corners + delta_corners));
        seen.insert((row, col));
    }

    // Sides == Corners
    println!("{}", region.values().map(|(area, corners)| area * *corners).sum::<i64>());
}
