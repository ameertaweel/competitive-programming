// +------+
// | NOTE |
// +------+
//
// My Solution To Part 2 Is Tailored To My Specific Input

use std::io;
use std::convert::TryInto;

fn main() {
    read_line();

    let mut b = read_register();
    let mut c = read_register();

    read_line();

    let program = read_program();

    let mut a_initial = 1 << 45;

    'outer: loop {
        let mut a = a_initial;

        let mut pc = 0;

        let mut output: Vec<u64> = Vec::new();
        
        while pc < program.len() {
            let instruction = program[pc];
            let operand = program[pc + 1];

            match instruction {
                0 => {
                    let numerator = a;
                    let denominator = 2_u64.pow(combo(operand, a, b, c).try_into().unwrap());
                    a = numerator / denominator;
                },
                1 => {
                    b = b ^ operand;
                },
                2 => {
                    b = combo(operand, a, b, c) % 8;
                },
                3 => {
                    if a != 0 {
                        pc = operand as usize;
                        continue;
                    }
                },
                4 => {
                    b = b ^ c;
                },
                5 => {
                    output.push(combo(operand, a, b, c) % 8);
                },
                6 => {
                    let numerator = a;
                    let denominator = 2_u64.pow(combo(operand, a, b, c).try_into().unwrap());
                    b = numerator / denominator;
                },
                7 => {
                    let numerator = a;
                    let denominator = 2_u64.pow(combo(operand, a, b, c).try_into().unwrap());
                    c = numerator / denominator;
                },
                _ => {
                    panic!("Invalid Instruction")
                },
            }

            pc += 2;
        }

        for (i, (p, o)) in program.iter().zip(output.iter()).rev().enumerate() {
            if p == o { continue; }

            let shift = 45 - 3 * i;
            a_initial = ((a_initial >> shift) + 1) << shift;

            continue 'outer;
        }

        break;
    }

    println!("{}", a_initial);
}

fn read_register() -> u64 {
    let line = read_line();
    let l = "Register X: ".len();
    return line[l..].parse().unwrap();
}

fn read_program() -> Vec<u64> {
    let line = read_line();
    let l = "Program: ".len();
    return line[l..].split(",").map(|c| c.parse().unwrap()).collect();
}

fn combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0 => operand,
        1 => operand,
        2 => operand,
        3 => operand,
        4 => a,
        5 => b,
        6 => c,
        7 => panic!("ERROR: Reserved Combo Operand 7"),
        _ => panic!("ERROR: Invalid Combo Operand"),
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    return line.trim().into();
}
