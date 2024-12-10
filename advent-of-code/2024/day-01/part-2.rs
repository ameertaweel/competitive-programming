use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut list_1: Vec<u64> = Vec::new();
    let mut list_2: HashMap<u64, u64> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let locs: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        list_1.push(locs[0]);

        list_2.insert(
            locs[1],
            list_2.get(&locs[1]).unwrap_or(&0) + 1
        );
    }
    list_1.sort();

    let list_1 = list_1;
    let list_2 = list_2;

    let similarity = list_1.iter()
        .map(|i| (*i) * list_2.get(&i).unwrap_or(&0))
        .sum::<u64>();

    println!("{}", similarity);
}
