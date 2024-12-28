use std::collections::HashMap;
use std::io::{self, BufRead};

enum Gate {
    AND(usize, usize),
     OR(usize, usize),
    XOR(usize, usize),
}

fn main() {
    let stdin = io::stdin();

    let (name_to_idx, mut vals, gates) = {
        let mut name_to_idx = HashMap::new();

        let mut get_name_idx = |name: &str| -> usize {
            let idx = name_to_idx.len();
            *name_to_idx.entry(name.to_string()).or_insert(idx)
        };

        let mut vals  = HashMap::new();
        let mut gates = HashMap::new();
        let mut still_initial = true;

        for line in stdin.lock().lines() {
            let line = line.unwrap();

            if line.len() == 0 {
                still_initial = false;
                continue;
            }

            if still_initial {
                let mut parts = line.split(": ");
                let (name, value): (&str, bool) = match (parts.next(), parts.next()) {
                    (Some(name), Some(value)) => (name, value == "1"),
                    _ => panic!("Invalid Input"),
                };

                let idx = get_name_idx(name);
                vals.insert(idx, value);
                continue;
            }

            let mut parts1 = line.split(" -> ");
            let (gate, output) = match (parts1.next(), parts1.next()) {
                (Some(gate), Some(output)) => (gate, output),
                _ => panic!("Invalid Input"),
            };

            let out_idx = get_name_idx(output);

            let mut parts2 = gate.split(" ");
            let gate = match (parts2.next(), parts2.next(), parts2.next()) {
                (Some(in1), Some("AND"), Some(in2)) => Gate::AND(get_name_idx(in1), get_name_idx(in2)),
                (Some(in1), Some("OR" ), Some(in2)) => Gate::OR( get_name_idx(in1), get_name_idx(in2)),
                (Some(in1), Some("XOR"), Some(in2)) => Gate::XOR(get_name_idx(in1), get_name_idx(in2)),
                _ => panic!("Invalid Input"),
            };

            gates.insert(out_idx, gate);
        }
        (name_to_idx, vals, gates)
    };

    let mut z: Vec<_> = name_to_idx.keys().filter(|&k| k.starts_with('z')).collect();
    z.sort_by(|a, b| b.cmp(a));
    let mut bitstring = vec![' '; z.len()];
    for (i, name) in z.iter().enumerate() {
        let idx = *name_to_idx.get(&name.to_string()).unwrap();
        bitstring[i] = if eval(idx, &mut vals, &gates) { '1' } else { '0' };
    }

    let bitstring = bitstring.iter().collect::<String>();

    let val = u64::from_str_radix(&bitstring, 2).unwrap();

    println!("{}", val);
}

fn eval(idx: usize, vals: &mut HashMap<usize, bool>, gates: &HashMap<usize, Gate>) -> bool {
    if vals.contains_key(&idx) {
        return *vals.get(&idx).unwrap();
    }

    let value = match *gates.get(&idx).unwrap() {
        Gate::AND(in1, in2) => eval(in1, vals, gates) & eval(in2, vals, gates),
        Gate::OR( in1, in2) => eval(in1, vals, gates) | eval(in2, vals, gates),
        Gate::XOR(in1, in2) => eval(in1, vals, gates) ^ eval(in2, vals, gates),
    };

    vals.insert(idx, value);

    return value;
}
