use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();

    let mut sum: u64 = 0;
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.contains('|') {
            let rule: Vec<u64> = line
                .split('|')
                .map(|s| s.parse().unwrap())
                .collect();
            let before = rule[0];
            let after  = rule[1];

            rules.entry(before).or_insert(HashSet::new());

            rules.get_mut(&before).unwrap().insert(after);
            continue;
        }

        if !line.contains(',') { continue; }

        let mut seen: HashSet<u64> = HashSet::new();

        let mut updated_pages: Vec<u64> = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut mark_for_fix = false;

        'outer: for page in &updated_pages {
            for next in rules.get(&page).unwrap_or(&HashSet::new()) {
                if seen.contains(next) {
                    mark_for_fix = true;
                    break 'outer;
                }
            }
            seen.insert(*page);
        }

        if !mark_for_fix { continue; }

        updated_pages.sort_by(|a, b| {
            match rules.get(&a) {
                Some(rules_a) => {
                    if rules_a.contains(b) { return Ordering::Less; }
                }
                None => {}
            }
            match rules.get(&b) {
                Some(rules_b) => {
                    if rules_b.contains(a) { return Ordering::Greater; }
                }
                None => {}
            }
            return Ordering::Equal;
        });

        let middle_page = updated_pages[updated_pages.len() / 2];
        sum += middle_page;
    }

    println!("{}", sum);
}
