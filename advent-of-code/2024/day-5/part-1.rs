use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();

    let mut sum: u64 = 0;
    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();

    'outer: for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.contains('|') {
            let rule: Vec<u64> = line
                .split('|')
                .map(|s| s.parse().unwrap())
                .collect();
            let before = rule[0];
            let after  = rule[1];

            rules.entry(before).or_insert(Vec::new());

            rules.get_mut(&before).unwrap().push(after);
            continue;
        }

        if !line.contains(',') { continue; }

        let mut seen: HashSet<u64> = HashSet::new();

        let updated_pages: Vec<u64> = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        for page in &updated_pages {
            for next in rules.get(&page).unwrap_or(&Vec::new()) {
                if seen.contains(next) { continue 'outer; }
            }
            seen.insert(*page);
        }

        let middle_page = updated_pages[updated_pages.len() / 2];
        sum += middle_page;
    }

    println!("{}", sum);
}
