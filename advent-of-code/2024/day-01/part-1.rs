use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut list_1: Vec<u64> = Vec::new();
    let mut list_2: Vec<u64> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let locs: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        list_1.push(locs[0]);
        list_2.push(locs[1]);
    }
    list_1.sort();
    list_2.sort();

    let list_1 = list_1;
    let list_2 = list_2;

    let diff = list_1.iter().zip(list_2.iter())
        .map(|(a, b)| (*a).abs_diff(*b))
        .sum::<u64>();

    println!("{}", diff);
}
