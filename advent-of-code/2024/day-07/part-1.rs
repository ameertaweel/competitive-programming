use std::io::{self, BufRead};

enum Operator {
    ADD,
    MUL,
}

fn main() {
    let stdin = io::stdin();

    let mut calibration_result = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let eq: Vec<&str> = line.split(": ").collect();

        let lhs: u64 = eq[0].parse().unwrap();
        let rhs: Vec<u64> = eq[1].split_whitespace().map(|s| s.parse().unwrap()).collect();

        let mut stack: Vec<(usize, u64, Operator)> = Vec::new();
        stack.push((1, rhs[0], Operator::ADD));
        stack.push((1, rhs[0], Operator::MUL));

        while stack.len() > 0 {
            let (idx, acc, operator) = stack.pop().unwrap();

            if acc > lhs { continue; }

            if idx == rhs.len() {
                match acc {
                    n if n == lhs => {
                        calibration_result += lhs;
                        break;
                    },
                    _ => continue,
                };
            }

            let acc = match operator {
                Operator::ADD => acc + rhs[idx],
                Operator::MUL => acc * rhs[idx],
            };

            stack.push((idx + 1, acc, Operator::ADD));
            stack.push((idx + 1, acc, Operator::MUL));
        }
    }

    println!("{}", calibration_result);
}
