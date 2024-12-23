use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let (mut network, idx_to_name) = {
        let mut node_idx = HashMap::new();
        let mut idx_node = HashMap::new();
        let mut network: HashMap<usize, HashSet<usize>> = HashMap::new();
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
                    network.insert(idx, HashSet::new());
                    idx
                }
            };

            let i0 = get_node_idx(n0);
            let i1 = get_node_idx(n1);

            network.get_mut(&i0).unwrap().insert(i1);
            network.get_mut(&i1).unwrap().insert(i0);
        }
        for (node, idx) in node_idx {
            idx_node.insert(idx, node);
        }
        (network, idx_node)
    };

    let most_connected_vertex = |network: &HashMap<usize, HashSet<usize>>| -> usize {
        let v: Vec<_> = network.keys().cloned().collect();
        *v.iter().max_by(|a, b| {
            let degree_a = network.get(a).unwrap().len();
            let degree_b = network.get(b).unwrap().len();
            degree_a.cmp(&degree_b)
        }).unwrap()
    };

    let mut max_clique = HashSet::new();
    let mut m = most_connected_vertex(&network);
    let mut max_possible_clique_size = network.get(&m).unwrap().len();

    while max_possible_clique_size > max_clique.len() {
        let mut clique = HashSet::new();
        clique.insert(m);

        'outer: for &i in network.get(&m).unwrap() {
            for j in &clique {
                if !network.get(&j).unwrap().contains(&i) { continue 'outer; }
            }
            clique.insert(i);
        }

        if clique.len() > max_clique.len() {
            max_clique = clique;
        }

        for i in network.get(&m).unwrap().clone() {
            network.get_mut(&i).unwrap().remove(&m);
        }

        network.remove(&m);

        m = most_connected_vertex(&network);
        max_possible_clique_size = network.get(&m).unwrap().len();
    }

    let mut clique_node_names = max_clique.iter().map(|i| idx_to_name[i].clone()).collect::<Vec<_>>();
    clique_node_names.sort();
    let password = clique_node_names.join(",");
    println!("{}", password);
}
