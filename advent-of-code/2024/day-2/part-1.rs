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

        if levels.len() <= 1 {
            safe_reports += 1;
            continue;
        }

        let diff_sign = (levels[1] - levels[0]).signum();

        if diff_sign == 0 {
            continue;
        }

        for i in 1..levels.len() {
            let prev_level = levels[i - 1];
            let curr_level = levels[i];

            let diff = curr_level - prev_level;
            if diff.signum() != diff_sign {
                continue 'outer;
            }
            if diff.abs() > 3 {
                continue 'outer;
            }
        }

        safe_reports += 1;
    }

    println!("{}", safe_reports);
}
