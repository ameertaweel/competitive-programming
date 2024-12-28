// Semi-Manual Approach
// This Code Finds First Z Wire With Issues
// I Found The Right Swaps To Fix The Issue Manually

use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GateType { AND, OR, XOR }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    gate_type: GateType,
    input1: String,
    input2: String,
    outputs: Vec<String>,
}

fn main() {
    let (bits, gates) = parse_input();

    for i in 0..=bits {
        if verify_z(&format!("z{:02}", i), i, bits, &gates) {
            println!("GOOD: z{:02}", i);
        } else {
            println!("BAAD: z{:02}", i);
            break;
        }
    }
}

fn verify_z(name: &str, num: usize, bits: usize, gates: &HashMap<String, Gate>) -> bool {
    if !name.starts_with('z') {
        println!("VZ {}, {}: Wrong Name", name, num);
        return false;
    }
    let n = name[1..].parse::<usize>().unwrap();
    if n != num {
        println!("VZ {}, {}: Wrong Num {}", name, num, n);
        return false;
    }

    if num == 0 {
        return verify_early_xor(name, num, gates);
    }
    if num == bits {
        return verify_carry(name, num - 1, gates);
    }

    let gate = gates.get(name).unwrap();

    if gate.gate_type != GateType::XOR {
        println!("VZ {}, {}: Wrong Gate {:?}", name, num, gate.gate_type);
        return false;
    }

    return (verify_early_xor(&gate.input1, num, gates) && verify_carry(&gate.input2, num - 1, gates))
        || (verify_early_xor(&gate.input2, num, gates) && verify_carry(&gate.input1, num - 1, gates));
}

fn verify_early_xor(name: &str, num: usize, gates: &HashMap<String, Gate>) -> bool {
    let gate = gates.get(name).unwrap();

    if gate.input1 != format!("x{:02}", num) || gate.input2 != format!("y{:02}", num) {
        println!("VEX {}, {}: Wrong Inputs {}, {}", name, num, gate.input1, gate.input2);
        return false;
    }
    if gate.gate_type != GateType::XOR {
        println!("VEX {}, {}: Wrong Gate {:?}", name, num, gate.gate_type);
        return false;
    }
    return true;
}

fn verify_carry(name: &str, num: usize, gates: &HashMap<String, Gate>) -> bool {
    if num == 0 {
        return verify_early_and(name, num, gates);
    }

    let gate = gates.get(name).unwrap();

    if gate.gate_type != GateType::OR {
        println!("VC {}, {}: Wrong Gate {:?}", name, num, gate.gate_type);
        return false;
    }

    return (verify_early_and(&gate.input1, num, gates) && verify_later_and(&gate.input2, num, gates))
        || (verify_early_and(&gate.input2, num, gates) && verify_later_and(&gate.input1, num, gates));
}

fn verify_later_and(name: &str, num: usize, gates: &HashMap<String, Gate>) -> bool {
    let gate = gates.get(name).unwrap();

    if gate.gate_type != GateType::AND {
        println!("VLA {}, {}: Wrong Gate {:?}", name, num, gate.gate_type);
        return false;
    }

    return (verify_early_xor(&gate.input1, num, gates) && verify_carry(&gate.input2, num - 1, gates))
        || (verify_early_xor(&gate.input2, num, gates) && verify_carry(&gate.input1, num - 1, gates));
}

fn verify_early_and(name: &str, num: usize, gates: &HashMap<String, Gate>) -> bool {
    let gate = gates.get(name).unwrap();

    if gate.input1 != format!("x{:02}", num) || gate.input2 != format!("y{:02}", num) {
        println!("VEA {}, {}: Wrong Inputs {}, {}", name, num, gate.input1, gate.input2);
        return false;
    }
    if gate.gate_type != GateType::AND {
        println!("VEA {}, {}: Wrong Gate {:?}", name, num, gate.gate_type);
        return false;
    }
    return true;
}

fn parse_input() -> (usize, HashMap<String, Gate>) {
    let stdin = io::stdin();

    let mut gates = HashMap::new();

    let mut bits = 0;

    let mut still_initial = true;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            still_initial = false;
            continue;
        }

        if still_initial {
            bits += 1;
            continue;
        }

        let (output, gate) = parse_gate(&line);
        gates.insert(output, gate);
    }

    let gates_temp = gates.clone();
    for (output, gate) in &gates_temp {
        if !gate.input1.starts_with('x') && !gate.input1.starts_with('y') {
            gates.get_mut(&gate.input1).unwrap().outputs.push(output.clone());
        }
        if !gate.input2.starts_with('x') && !gate.input2.starts_with('y') {
            gates.get_mut(&gate.input2).unwrap().outputs.push(output.clone());
        }
    }

    return (bits / 2, gates);
}

fn parse_gate(line: &str) -> (String, Gate) {
    let mut parts1 = line.split(" -> ");
    let (gate, output) = match (parts1.next(), parts1.next()) {
        (Some(gate), Some(output)) => (gate, output),
        _ => panic!("Invalid Input"),
    };

    let mut parts2 = gate.split(" ");
    let (input1, type_, input2) = match (parts2.next(), parts2.next(), parts2.next()) {
        (Some(input1), Some(type_), Some(input2)) => {
            (input1, type_, input2)
        },
        _ => panic!("Invalid Input"),
    };
    let (input1, input2) = (std::cmp::min(input1, input2), std::cmp::max(input1, input2));
    let gate_type = match type_ {
        "AND" => GateType::AND,
        "OR"  => GateType::OR,
        "XOR" => GateType::XOR,
          _   => panic!("Invalid Input"),
    };

    return (
        output.to_string(),
        Gate {
            gate_type: gate_type,
            input1: input1.to_string(),
            input2: input2.to_string(),
            outputs: Vec::new(),
        }
    );
}
