use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let (network, starts_with_t) = {
        let mut node_idx      = HashMap::new();
        let mut network       = Vec::new();
        let mut starts_with_t = Vec::new();
        for line in stdin.lock().lines() {
            let line = line.unwrap();

            let nodes: Vec<String> = line.split("-").map(|c| c.to_string()).collect();
            let n0 = &nodes[0];
            let n1 = &nodes[1];

            let mut get_node_idx = |name: &String| -> usize {
                if node_idx.contains_key(name) {
                    *node_idx.get(name).unwrap()
                } else {
                    let idx = network.len();
                    node_idx.insert(name.clone(), idx);
                    starts_with_t.push(&name[0..1] == "t");
                    network.push(HashSet::new());
                    idx
                }
            };

            let i0 = get_node_idx(n0);
            let i1 = get_node_idx(n1);

            network[i0].insert(i1);
            network[i1].insert(i0);
        }
        (network, starts_with_t)
    };

    let n = network.len();

    let mut special_lan_parties = 0;
    for i0 in 0..n {
        for i1 in (i0 + 1)..n {
            if !network[i0].contains(&i1) { continue; }
            for i2 in (i1 + 1)..n {
                if !network[i0].contains(&i2) || !network[i1].contains(&i2) { continue; }
                if !starts_with_t[i0] && !starts_with_t[i1] && !starts_with_t[i2] { continue; }
                special_lan_parties += 1;
            }
        }
    }

    println!("{}", special_lan_parties);
}
