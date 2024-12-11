use std::io;
use std::collections::HashMap;

const BLINKS: u64 = 75;

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

    let mut curr_map: HashMap<u64, u64> = HashMap::new();
    for s in stones.into_iter() {
        *curr_map.entry(s).or_insert(0) += 1;
    }

    for _ in 0..BLINKS {
        let mut next_map: HashMap<u64, u64> = HashMap::new();
        for (stone, count) in &curr_map {
            let stone = *stone;
            if stone == 0 {
                *next_map.entry(1).or_insert(0) += count;
                continue;
            }

            let dgts = digits(stone);

            if dgts % 2 == 0 {
                *next_map.entry(left_half(stone)).or_insert(0) += count;
                *next_map.entry(right_half(stone)).or_insert(0) += count;
                continue;
            }

            *next_map.entry(stone * 2024).or_insert(0) += count;
        }
        curr_map = next_map;
    }

    println!("{}", curr_map.values().sum::<u64>());
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
