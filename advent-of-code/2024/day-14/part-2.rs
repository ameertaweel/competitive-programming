use std::io::{self, BufRead};

const ROWS: i64 = 103;
const COLS: i64 = 101;
const SECS: i64 = 10000;

fn main() {
    let stdin = io::stdin();

    let mut robots: Vec<(i64, i64, i64, i64)> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let (x, y, vx, vy) = parse_input(&line);

        robots.push((x, y, vx, vy));
    }

    let mut min_asymmetrical = ROWS * COLS;
    let mut easter_egg_tree = vec![vec![' '; COLS as usize]; ROWS as usize];
    let mut easter_egg_second = -1;

    for sec in 0..SECS {
        let mut grid = vec![vec![' '; COLS as usize]; ROWS as usize];

        for robot in robots.iter_mut() {
            let (ref mut x, ref mut y, ref vx, ref vy) = robot;

            *x = (*x + *vx + COLS) % COLS;
            *y = (*y + *vy + ROWS) % ROWS;

            grid[*y as usize][*x as usize] = '*';
        }


        let mut asymmetrical = 0;
        for row in 0..ROWS {
            for col in 0..COLS {
                if grid[row as usize][col as usize] != grid[row as usize][(COLS - 1 - col) as usize] {
                    asymmetrical += 1;
                }
            }
        }

        if asymmetrical < min_asymmetrical {
            min_asymmetrical = asymmetrical;
            easter_egg_tree = grid;
            easter_egg_second = sec + 1;
        }
    }


    println!("We Think We Found The Easter Egg!");

    for row in 0..ROWS {
        for col in 0..COLS {
            print!("{}", easter_egg_tree[row as usize][col as usize]);
        }
        println!("");
    }

    println!("Asymmetry Score: {}", min_asymmetrical);
    println!("Happened @ Second: {}", easter_egg_second);
}

fn parse_input(s: &str) -> (i64, i64, i64, i64) {
    let info: Vec<_> = s.split(" ").collect();
    let pos: Vec<i64> = info[0][2..].split(",").map(|x| x.parse().unwrap()).collect();
    let vel: Vec<i64> = info[1][2..].split(",").map(|x| x.parse().unwrap()).collect();
    return (pos[0], pos[1], vel[0], vel[1]);
}
