use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut sum: u64 = 0;

    enum State {
        CORRUPT,
        OPEN,
        NUM1,
        COMMA,
        NUM2
    }

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let mut i = 0;

        let mut state = State::CORRUPT;
        let mut start = 0;

        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            match state {
                State::CORRUPT => {
                    // look for the substring "mul("
                    if c != 'm' {
                        i += 1;
                        continue;
                    }
                    if line.len() - i < 4 {
                        i += 1;
                        continue;
                    }
                    if &line[i..(i + 4)] != "mul(" {
                        i += 1;
                        continue;
                    }
                    i += 4;
                    state = State::OPEN;
                }
                State::OPEN => {
                    if "0123456789".contains(c) {
                        start = i;
                        state = State::NUM1;
                    } else {
                        state = State::CORRUPT;
                    }
                    i += 1;
                }
                State::NUM1 => {
                    if "0123456789".contains(c) {
                    } else if c == ',' {
                        state = State::COMMA;
                    } else {
                        state = State::CORRUPT;
                    }
                    i += 1;
                }
                State::COMMA => {
                    if "0123456789".contains(c) {
                        state = State::NUM2;
                    } else {
                        state = State::CORRUPT;
                    }
                    i += 1;
                }
                State::NUM2 => {
                    if !("0123456789".contains(c)) {
                        state = State::CORRUPT;
                        if c == ')' {
                            let nums: Vec<u64> = (&line[start..i])
                                .split(',')
                                .map(|s| s.parse().unwrap())
                                .collect();
                            sum += nums[0] * nums[1];
                        }
                    }
                    i += 1;
                }
            }
        }
    }

    println!("{}", sum);
}
