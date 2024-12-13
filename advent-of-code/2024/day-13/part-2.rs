use std::io::{self, BufRead};

const COST_A: i128 = 3;
const COST_B: i128 = 1;
const CORRECTION: i128 = 10000000000000;

enum State { A, B, P }

fn main() {
    let stdin = io::stdin();

    let mut machines: Vec<(i128, i128, i128, i128, i128, i128)> = Vec::new();
    {
        let mut state = State::A;
        let mut ax: i128 = 0;
        let mut ay: i128 = 0;
        let mut bx: i128 = 0;
        let mut by: i128 = 0;
        let mut px: i128;
        let mut py: i128;

        for line in stdin.lock().lines() {
            let line = line.unwrap();

            if line.len() == 0 { continue; }

            match state {
                State::A => {
                    (ax, ay) = parse_input(&line, 10, false);
                    state = State::B;
                },
                State::B => {
                    (bx, by) = parse_input(&line, 10, false);
                    state = State::P;
                },
                State::P => {
                    (px, py) = parse_input(&line, 7, true);
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

        let determinant = ax * by - bx * ay;

        if determinant == 0 { continue; } // Inverse Doesn't Exist

        let presses_a = (px * by - py * bx) / determinant;
        let presses_b = (py * ax - px * ay) / determinant;

        // Only whole presses are allowed
        if presses_a * ax + presses_b * bx != px { continue; }
        if presses_a * ay + presses_b * by != py { continue; }

        tokens += presses_a * COST_A + presses_b * COST_B;
    }

    println!("{}", tokens);
}

fn parse_input(s: &str, prefix: usize, correct: bool) -> (i128, i128) {
    let nums: Vec<i128> = s[prefix..].split(", ").map(|x| x[2..].parse().unwrap()).collect();
    return if correct {
        (nums[0] + CORRECTION, nums[1] + CORRECTION)
    } else {
        (nums[0], nums[1])
    };
}
