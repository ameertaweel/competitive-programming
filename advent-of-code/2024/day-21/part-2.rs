use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let numeric_patterns: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    let mut complexity = 0;

    for pattern in numeric_patterns.iter() {
        let mut cache: HashMap<(char, usize, usize, u64), u64> = HashMap::new();

        let (mut row, mut col) = (3, 2);

        let mut count = 0;
        for c in pattern.chars() {
            let (partial_count, row_next, col_next) = numeric_count(c, row, col, 0, &mut cache);
            count += partial_count;
            (row, col) = (row_next, col_next);
        }

        let numeric_part: u64 = pattern[0..(pattern.len() - 1)].parse().unwrap();

        complexity += count * numeric_part;
    }

    println!("{}", complexity);
}

fn numeric_count(
    el: char, row_curr: usize, col_curr: usize, level: u64,
    cache: &mut HashMap<(char, usize, usize, u64), u64>
) -> (u64, usize, usize) {
    let (row_next, col_next) = match el {
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

    let mut found = false;
    let mut min_count = 0;
    let mut stack = Vec::new();
    stack.push((row_diff, col_diff, 0, 2, 0));

    while stack.len() > 0 {
        let (row_diff, col_diff, row_c, col_c, count) = stack.pop().unwrap();

        if row_diff == 0 && col_diff == 0 {
            let (partial_count, _, _) = recursive_directional_count('A', row_c, col_c, level + 1, cache);

            if !found || count + partial_count < min_count {
                min_count = count + partial_count;
                found = true;
            }

            continue;
        }

        if col_diff != 0 {
            let c = if col_diff.signum() == 1 { '>' } else { '<' };
            let (partial_count, row_n, col_n) = recursive_directional_count(c, row_c, col_c, level + 1, cache);
            let (row_diff_n, col_diff_n) = (row_diff, col_diff - col_diff.signum());
            let (row_curr_n, col_curr_n) = (row_next as i64 - row_diff_n, col_next as i64 - col_diff_n);
            if (row_n != 0 || col_n != 0) && (row_curr_n != 3 || col_curr_n != 0) {
                stack.push((row_diff_n, col_diff_n, row_n, col_n, count + partial_count));
            }
        }

        if row_diff != 0 {
            let c = if row_diff.signum() == 1 { 'v' } else { '^' };
            let (partial_count, row_n, col_n) = recursive_directional_count(c, row_c, col_c, level + 1, cache);
            let (row_diff_n, col_diff_n) = (row_diff - row_diff.signum(), col_diff);
            let (row_curr_n, col_curr_n) = (row_next as i64 - row_diff_n, col_next as i64 - col_diff_n);
            if (row_n != 0 || col_n != 0) && (row_curr_n != 3 || col_curr_n != 0) {
                stack.push((row_diff_n, col_diff_n, row_n, col_n, count + partial_count));
            }
        }
    }

    return (min_count, row_next, col_next);
}

fn recursive_directional_count(
    el: char, row_curr: usize, col_curr: usize, level: u64,
    cache: &mut HashMap<(char, usize, usize, u64), u64>
) -> (u64, usize, usize) {
    let (row_next, col_next) = match el {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _  => panic!("Invalid Directional Pattern"),
    };

    if level == 26 { return (1, row_next, col_next); }

    if cache.contains_key(&(el, row_curr, col_curr, level)) {
        return (*cache.get(&(el, row_curr, col_curr, level)).unwrap(), row_next, col_next);
    }
    
    let (row_diff, col_diff) = ((row_next as i64 - row_curr as i64), (col_next as i64 - col_curr as i64));

    let mut found = false;
    let mut min_count = 0;
    let mut stack = Vec::new();
    stack.push((row_diff, col_diff, 0, 2, 0));

    while stack.len() > 0 {
        let (row_diff, col_diff, row_c, col_c, count) = stack.pop().unwrap();

        if row_diff == 0 && col_diff == 0 {
            let (partial_count, _, _) = recursive_directional_count('A', row_c, col_c, level + 1, cache);

            if !found || count + partial_count < min_count {
                min_count = count + partial_count;
                found = true;
            }

            continue;
        }

        if col_diff != 0 {
            let c = if col_diff.signum() == 1 { '>' } else { '<' };
            let (partial_count, row_n, col_n) = recursive_directional_count(c, row_c, col_c, level + 1, cache);
            let (row_diff_n, col_diff_n) = (row_diff, col_diff - col_diff.signum());
            let (row_curr_n, col_curr_n) = (row_next as i64 - row_diff_n, col_next as i64 - col_diff_n);
            if (row_n != 0 || col_n != 0) && (row_curr_n != 0 || col_curr_n != 0) {
                stack.push((row_diff_n, col_diff_n, row_n, col_n, count + partial_count));
            }
        }

        if row_diff != 0 {
            let c = if row_diff.signum() == 1 { 'v' } else { '^' };
            let (partial_count, row_n, col_n) = recursive_directional_count(c, row_c, col_c, level + 1, cache);
            let (row_diff_n, col_diff_n) = (row_diff - row_diff.signum(), col_diff);
            let (row_curr_n, col_curr_n) = (row_next as i64 - row_diff_n, col_next as i64 - col_diff_n);
            if (row_n != 0 || col_n != 0) && (row_curr_n != 0 || col_curr_n != 0) {
                stack.push((row_diff_n, col_diff_n, row_n, col_n, count + partial_count));
            }
        }
    }

    cache.insert((el, row_curr, col_curr, level), min_count);

    return (min_count, row_next, col_next);
}
