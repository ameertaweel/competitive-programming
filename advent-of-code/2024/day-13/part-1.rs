use std::io::{self, BufRead};

const MAX_PRESSES: u64 = 100;
const COST_A: u64 = 3;
const COST_B: u64 = 1;

enum State { A, B, P }

fn main() {
    let stdin = io::stdin();

    let mut machines: Vec<(u64, u64, u64, u64, u64, u64)> = Vec::new();
    {
        let mut state = State::A;
        let mut ax: u64 = 0;
        let mut ay: u64 = 0;
        let mut bx: u64 = 0;
        let mut by: u64 = 0;
        let mut px: u64;
        let mut py: u64;

        for line in stdin.lock().lines() {
            let line = line.unwrap();

            if line.len() == 0 { continue; }

            match state {
                State::A => {
                    (ax, ay) = parse_input(&line, 10);
                    state = State::B;
                },
                State::B => {
                    (bx, by) = parse_input(&line, 10);
                    state = State::P;
                },
                State::P => {
                    (px, py) = parse_input(&line, 7);
                    machines.push((ax, ay, bx, by, px, py));
                    state = State::A;
                },
            }
        }
    }
    let machines = machines;

    let mut tokens = 0;

    for machine in machines {
        let (ax, ay, bx, by, px, py) = machine;

        let mut min_tokens = MAX_PRESSES * (COST_A + COST_B);
        let mut found = false;

        for press_a in 0..MAX_PRESSES {
            for press_b in 0..MAX_PRESSES {
                if press_a * ax + press_b * bx == px && press_a * ay + press_b * by == py {
                    found = true;
                    let tokens = press_a * COST_A + press_b * COST_B;
                    if tokens < min_tokens {
                        min_tokens = tokens;
                    }
                }
            }
        }

        if found {
            tokens += min_tokens;
        }
    }

    println!("{}", tokens);
}

fn parse_input(s: &str, prefix: usize) -> (u64, u64) {
    let nums: Vec<u64> = s[prefix..].split(", ").map(|x| x[2..].parse().unwrap()).collect();
    return (nums[0], nums[1]);
}
