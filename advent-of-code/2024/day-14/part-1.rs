use std::io::{self, BufRead};

const ROWS: i64 = 103;
const COLS: i64 = 101;
const SECS: i64 = 100;

fn main() {
    let stdin = io::stdin();

    let mut robots: Vec<(i64, i64, i64, i64)> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let (x, y, vx, vy) = parse_input(&line);

        robots.push((x, y, vx, vy));
    }


    for _ in 0..SECS {
        for robot in robots.iter_mut() {
            let (ref mut x, ref mut y, ref vx, ref vy) = robot;

            *x = (*x + *vx + COLS) % COLS;
            *y = (*y + *vy + ROWS) % ROWS;
        }
    }
    let robots = robots;

    let mut first  = 0;
    let mut second = 0;
    let mut third  = 0;
    let mut fourth = 0;

    for robot in robots.iter() {
        let (x, y, _, _) = robot;

        if *x < COLS / 2 && *y < ROWS / 2 { first  += 1; }
        if *x > COLS / 2 && *y < ROWS / 2 { second += 1; }
        if *x > COLS / 2 && *y > ROWS / 2 { third  += 1; }
        if *x < COLS / 2 && *y > ROWS / 2 { fourth += 1; }
    }

    let safety_factor = first * second * third * fourth;
    println!("{}", safety_factor);
}

fn parse_input(s: &str) -> (i64, i64, i64, i64) {
    let info: Vec<_> = s.split(" ").collect();
    let pos: Vec<i64> = info[0][2..].split(",").map(|x| x.parse().unwrap()).collect();
    let vel: Vec<i64> = info[1][2..].split(",").map(|x| x.parse().unwrap()).collect();
    return (pos[0], pos[1], vel[0], vel[1]);
}
