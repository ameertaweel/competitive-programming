use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();

    // Read Input
    let mut signals: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut rows: i64 = 0;
    let mut cols: i64 = 0;
    for (row, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        rows += 1;
        cols = line.len() as i64;
        for (col, char) in line.chars().enumerate() {
            if char == '.' { continue; }

            signals.entry(char).or_insert(Vec::new());

            signals.get_mut(&char).unwrap().push((row as i64, col as i64));
        }
    }
    let signals = signals;

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for (_signal, locations) in &signals {
        for i in 0..locations.len() {
            let loc1 = locations[i];
            for j in (i + 1)..locations.len() {
                let loc2 = locations[j];

                let delta_row = loc1.0 - loc2.0;
                let delta_col = loc1.1 - loc2.1;

                let antinode1 = (loc1.0 + delta_row, loc1.1 + delta_col);
                let antinode2 = (loc2.0 - delta_row, loc2.1 - delta_col);

                if within_grid(&antinode1, rows, cols) {
                    antinodes.insert(antinode1);
                }
                if within_grid(&antinode2, rows, cols) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

fn within_grid(pos: &(i64, i64), rows: i64, cols: i64) -> bool {
    return pos.0 >= 0 && pos.0 < cols && pos.1 >= 0 && pos.1 < rows;
}
