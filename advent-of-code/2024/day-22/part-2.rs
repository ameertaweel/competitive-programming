use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

const STEPS: u64 = 2000;

fn main() {
    let stdin = io::stdin();

    let secrets_initial: Vec<u64> = stdin.lock().lines().map(|line| line.unwrap().parse().unwrap()).collect();

    let mut sequences_bananas: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for secret in secrets_initial.iter() {
        let mut secret = *secret;
        let mut changes_history = VecDeque::new();
        let mut seen_sequences = HashSet::new();

        let mut price = secret % 10;
        for _ in 0..STEPS {
            secret = (secret ^ (secret *   64)) % 16777216;
            secret = (secret ^ (secret /   32)) % 16777216;
            secret = (secret ^ (secret * 2048)) % 16777216;

            let price_next = secret % 10;

            let change = (price_next as i64) - (price as i64);

            price = price_next;

            changes_history.push_back(change);

            if changes_history.len() > 4 {
                changes_history.pop_front().unwrap();
            }

            if changes_history.len() == 4 {
                let seq = (changes_history[0], changes_history[1], changes_history[2], changes_history[3]);
                if seen_sequences.contains(&seq) { continue; }
                sequences_bananas.insert(seq, *sequences_bananas.get(&seq).unwrap_or(&0) + price as i64);
                seen_sequences.insert(seq);
            }
        }
    }

    let max_banana = sequences_bananas.iter().map(|(_, &bananas)| bananas).max().unwrap();
    println!("{}", max_banana);
}
