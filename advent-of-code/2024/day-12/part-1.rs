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
    // (Origin Row, Origin Col), (Area, Perimeter)
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

        let (area, perimeter) = region.get(&(origin_row, origin_col)).unwrap_or(&(0, 0));
        let mut delta_perimeter = 4;

        let possible_next: Vec<(i64, i64)> = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];

        for (row, col) in possible_next {
            if row < 0 || row >= rows || col < 0 || col >= cols { continue; }

            if grid[row as usize][col as usize] != plant { continue; }

            delta_perimeter -= 1;

            if seen.contains(&(row, col)) { continue; }

            stack.push((origin_row, origin_col, row, col));
        }

        region.insert((origin_row, origin_col), (area + 1, perimeter + delta_perimeter));
        seen.insert((row, col));
    }

    println!("{}", region.values().map(|(area, perimeter)| area * *perimeter).sum::<i64>());
}
