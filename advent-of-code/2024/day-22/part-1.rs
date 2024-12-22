use std::io::{self, BufRead};

const STEPS: u64 = 2000;

fn main() {
    let stdin = io::stdin();

    let secrets_initial: Vec<u64> = stdin.lock().lines().map(|line| line.unwrap().parse().unwrap()).collect();

    let secrets_final: Vec<u64> = secrets_initial.iter().map(|secret| {
        let mut secret = *secret;
        for _ in 0..STEPS {
            secret = (secret ^ (secret *   64)) % 16777216;
            secret = (secret ^ (secret /   32)) % 16777216;
            secret = (secret ^ (secret * 2048)) % 16777216;
        }
        return secret;
    }).collect();

    println!("{}", secrets_final.iter().sum::<u64>());
}
