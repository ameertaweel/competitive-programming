use std::io::{self, BufRead};

const LOCK_KEY_SIZE: usize = 5;

fn main() {
    let stdin = io::stdin();

    let (raw_locks, raw_keys) = {
        let (mut raw_locks, mut raw_keys) = (Vec::new(), Vec::new());
        let (mut i, mut is_lock) = (0, true);

        for line in stdin.lock().lines() {
            let line = line.unwrap();

            if line.len() == 0 { continue; }

            if i == 0 {
                is_lock = line.contains('#');
                i += 1;
                continue;
            }

            if i == LOCK_KEY_SIZE + 1 {
                i = 0;
                continue;
            }

            if is_lock {
                raw_locks.push(line);
            } else {
                raw_keys.push(line);
            }
            i += 1;
        }
        (raw_locks, raw_keys)
    };

    let locks = parse(raw_locks);
    let keys  = parse(raw_keys);

    let mut valid = 0;
    for lock in &locks {
        'outer: for key in &keys {
            for (l, k) in lock.iter().zip(key.iter()) {
                if l + k > LOCK_KEY_SIZE { continue 'outer; }
            }
            valid += 1;
        }
    }

    println!("{:?}", valid);
}

fn parse(raw: Vec<String>) -> Vec<Vec<usize>> {
    raw.chunks(LOCK_KEY_SIZE).map(|chunk| {
        let mut h = vec![0; chunk[0].len()];

        for row in chunk {
            for (i, char) in row.chars().enumerate() {
                h[i] += if char == '#' { 1 } else { 0 };
            }
        }

        return h;
    }).collect()
}
