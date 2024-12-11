use std::io;
use std::collections::VecDeque;

const BLINKS: u64 = 25;

fn main() {
    let stdin = io::stdin();

    // Read Input
    let mut stones = String::new();
    stdin.read_line(&mut stones).unwrap();
    let stones: Vec<u64> = stones
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut queue = VecDeque::from(stones);

    for _ in 0..BLINKS {
        for _ in 0..queue.len() {
            let curr_mark = queue.pop_front().unwrap();
            let curr_dgts = digits(curr_mark);

            if curr_mark == 0 {
                queue.push_back(1);
            } else if curr_dgts % 2 == 0 {
                queue.push_back(left_half(curr_mark));
                queue.push_back(right_half(curr_mark));
            } else {
                queue.push_back(curr_mark * 2024);
            }
        }
    }

    println!("{}", queue.len());
}

fn digits(n: u64) -> u64 {
    if n == 0 { return 0; }

    let mut l = 0;
    let mut v = 1;
    while v <= n {
        l += 1;
        v *= 10;
    };
    return l;
}

fn left_half(n: u64) -> u64 {
    let mut n = n;
    let mut d = digits(n);
    let h = d / 2;

    while d > h {
        n /= 10;
        d -= 1;
    }

    return n;
}

fn right_half(n: u64) -> u64 {
    let mut pow = 1;
    for _ in 0..(digits(n) / 2) {
        pow *= 10;
    }
    return n - (left_half(n) * pow);
}
