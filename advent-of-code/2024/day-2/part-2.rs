use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut safe_reports: u64 = 0;

    'outer: for line in stdin.lock().lines() {
        let line = line.unwrap();
        let levels: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if levels.len() <= 2 {
            safe_reports += 1;
            continue;
        }

        let mut stack: Vec<(i64, usize, usize, bool)> = Vec::new();

        stack.push((
            (levels[2] - levels[1]).signum(),
            1, 2,
            true
        ));
        stack.push((
            (levels[2] - levels[0]).signum(),
            0, 2,
            true
        ));
        stack.push((
            (levels[1] - levels[0]).signum(),
            0, 1,
            false
        ));

        while stack.len() > 0 {
            let (diff_sign, i, j, used_dampener) = stack.pop().unwrap();

            let prev_level = levels[i];
            let curr_level = levels[j];

            if !is_safe_transition(prev_level, curr_level, diff_sign) {
                if used_dampener {
                    continue;
                }
                if i == 0 {
                    continue;
                }
                if j == levels.len() - 1 {
                    safe_reports += 1;
                    continue 'outer;
                }
                stack.push((diff_sign, i - 1, j, true));
                stack.push((diff_sign, i, j + 1, true));
                continue;
            }

            if j == levels.len() - 1 {
                safe_reports += 1;
                continue 'outer;
            }

            stack.push((diff_sign, j, j + 1, used_dampener));
        }
    }

    println!("{}", safe_reports);
}

fn is_safe_transition(level_a: i64, level_b: i64, diff_sign: i64) -> bool {
    if diff_sign == 0 {
        return false;
    }
    let diff = level_b - level_a;
    if diff.signum() != diff_sign {
        return false;
    }
    if diff.abs() > 3 {
        return false;
    }
    return true;
}
