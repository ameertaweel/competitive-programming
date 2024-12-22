// +------+
// | NOTE |
// +------+
//
// My Solution To Part 1 Is Wrong. It Worked By Chance.
// For Correct Values In All Cases: Run Part 2 Code With `LEVELS = 2`.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let numeric_keypad_patterns: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    let directional_keypad_patterns: Vec<String> = numeric_keypad_patterns.iter().map(|numeric_pattern| {
        let mut directional_pattern = String::new();
        let (mut row_curr, mut col_curr) = (3, 2);
        for c in numeric_pattern.chars() {
            let (row_next, col_next) = match c {
                '0' => (3, 1),
                'A' => (3, 2),
                '1' => (2, 0),
                '2' => (2, 1),
                '3' => (2, 2),
                '4' => (1, 0),
                '5' => (1, 1),
                '6' => (1, 2),
                '7' => (0, 0),
                '8' => (0, 1),
                '9' => (0, 2),
                 _  => panic!("Invalid Numeric Pattern"),
            };

            let (row_diff, col_diff) = ((row_next as i64 - row_curr as i64), (col_next as i64 - col_curr as i64));

            if (row_curr == 3 && col_curr == 2 && col_diff == -2) || (row_curr == 3 && col_curr == 1 && col_diff == -1) { // Don't Go Over Gap
                if row_diff > 0 {
                    directional_pattern += &"v".repeat(row_diff as usize);
                } else if row_diff < 0 {
                    directional_pattern += &"^".repeat(-row_diff as usize);
                }

                if col_diff > 0 {
                    directional_pattern += &">".repeat(col_diff as usize);
                } else if col_diff < 0 {
                    directional_pattern += &"<".repeat(-col_diff as usize);
                }
            } else {
                if col_diff > 0 {
                    directional_pattern += &">".repeat(col_diff as usize);
                } else if col_diff < 0 {
                    directional_pattern += &"<".repeat(-col_diff as usize);
                }

                if row_diff > 0 {
                    directional_pattern += &"v".repeat(row_diff as usize);
                } else if row_diff < 0 {
                    directional_pattern += &"^".repeat(-row_diff as usize);
                }
            }

            directional_pattern += "A";

            (row_curr, col_curr) = (row_next, col_next);
        }

        return directional_pattern;
    }).collect();

    let next_directional_keypad_patterns = next_directional(&directional_keypad_patterns);
    let next_next_directional_keypad_patterns = next_directional(&next_directional_keypad_patterns);

    let numeric_parts: Vec<u64> = numeric_keypad_patterns.iter()
        .map(|pattern| pattern[0..(pattern.len() - 1)].parse().unwrap())
        .collect();
    let complexity_sum: u64 = next_next_directional_keypad_patterns.iter().zip(numeric_parts.iter())
        .map(|(&ref pattern, &numeric_part)| pattern.len() as u64 * numeric_part)
        .sum();
    println!("{}", complexity_sum);
}

fn next_directional(patterns: &Vec<String>) -> Vec<String> {
    let next_patterns: Vec<String> = patterns.iter().map(|pattern| {
        let mut next_pattern = String::new();
        let (mut row_curr, mut col_curr) = (0, 2);
        for c in pattern.chars() {
            let (row_next, col_next) = match c {
                '^' => (0, 1),
                'A' => (0, 2),
                '<' => (1, 0),
                'v' => (1, 1),
                '>' => (1, 2),
                 _  => panic!("Invalid Directional Pattern"),
            };

            let (row_diff, col_diff) = ((row_next as i64 - row_curr as i64), (col_next as i64 - col_curr as i64));

            if row_curr == 1 && col_curr == 0 && row_diff == -1 { // Don't Go Over Gap
                if col_diff > 0 {
                    next_pattern += &">".repeat(col_diff as usize);
                } else if col_diff < 0 {
                    next_pattern += &"<".repeat(-col_diff as usize);
                }

                if row_diff > 0 {
                    next_pattern += &"v".repeat(row_diff as usize);
                } else if row_diff < 0 {
                    next_pattern += &"^".repeat(-row_diff as usize);
                }
            } else {
                if row_diff > 0 {
                    next_pattern += &"v".repeat(row_diff as usize);
                } else if row_diff < 0 {
                    next_pattern += &"^".repeat(-row_diff as usize);
                }

                if col_diff > 0 {
                    next_pattern += &">".repeat(col_diff as usize);
                } else if col_diff < 0 {
                    next_pattern += &"<".repeat(-col_diff as usize);
                }
            }

            next_pattern += "A";

            (row_curr, col_curr) = (row_next, col_next);
        }

        return next_pattern;
    }).collect();
    return next_patterns;
}
