use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let (patterns, designs) = {
        let mut patterns = Trie::new();
        let mut designs = Vec::new();
        for (i, line) in stdin.lock().lines().enumerate() {
            let line = line.unwrap();

            if i == 0 {
                for word in line.split(", ") {
                    patterns.insert(word);
                }
                continue;
            }

            if i == 1 {
                continue;
            }

            designs.push(line);
        }
        (patterns, designs)
    };

    let mut cache = HashMap::new();
    let mut possible_designs_count = 0;
    for design in designs.iter() {
        possible_designs_count += count_ways_designs_possible(&design, &patterns, &mut cache);
    }

    println!("{}", possible_designs_count);
}

fn count_ways_designs_possible(
    design: &str,
    patterns: &Trie,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(&cached) = cache.get(design) {
        return cached;
    }
    let results = patterns.search(design);

    if results.len() == 0 {
        cache.insert(design.to_string(), 0);
        return 0;
    }

    let mut res = 0;
    for i in results {
        res += count_ways_designs_possible(&design[i..], patterns, cache)
    }
    cache.insert(design.to_string(), res);
    return res;
}

struct Trie {
    children: HashMap<char, Trie>,
    terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            terminal: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut trie = self;
        for c in word.chars() {
            trie = trie.children.entry(c).or_insert_with(Trie::new);
        }
        trie.terminal = true;
    }

    fn search(&self, word: &str) -> VecDeque<usize> {
        let mut trie = self;
        let mut results = VecDeque::new();

        for (i, c) in word.chars().enumerate() {
            if let Some(sub_trie) = trie.children.get(&c) {
                if sub_trie.terminal {
                    results.push_front(i + 1);
                }
                trie = sub_trie;
            } else {
                return results;
            }
        }

        return results;
    }
}
