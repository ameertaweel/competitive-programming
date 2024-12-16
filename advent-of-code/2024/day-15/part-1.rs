use std::io::{self, BufRead};

const MOVES: [(i64, i64); 4] = [
    ( 0, -1),
    ( 1,  0),
    ( 0,  1),
    (-1,  0),
];

enum Cell {
    EMPTY,
    BOX,
    WALL,
}

enum Move {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn main() {
    let stdin = io::stdin();

    let mut map: Vec<Vec<Cell>> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();
    let mut pos: (i64, i64) = (0, 0);
    let mut started_moves = false;
    for (row, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();

        if line.len() == 0 {
            started_moves = true;
            continue;
        }

        if !started_moves {
            map.push(line.chars().enumerate().map(|(col, char)| match char {
                '.' => Cell::EMPTY,
                'O' => Cell::BOX,
                '#' => Cell::WALL,
                '@' => {
                    pos = (col as i64, row as i64);
                    Cell::EMPTY
                },
                _   => panic!("Shouldn't Happen"),
            }).collect());
        } else {
            for c in line.chars() {
                match c {
                    '^' => moves.push(Move::UP),
                    '>' => moves.push(Move::RIGHT),
                    'v' => moves.push(Move::DOWN),
                    '<' => moves.push(Move::LEFT),
                     _  => panic!("Shouldn't Happen"),
                }
            }
        }
    }

    let rows = map.len();
    let cols = map[0].len();

    'outer: for m in moves.iter() {
        let (dx, dy) = match m {
            Move::UP    => MOVES[0],
            Move::RIGHT => MOVES[1],
            Move::DOWN  => MOVES[2],
            Move::LEFT  => MOVES[3],
        };

        let (x, y) = pos;

        let mut i = 1;

        loop {
            let x = x + i * dx;
            let y = y + i * dy;
            let cell = &map[y as usize][x as usize];
            match cell {
                Cell::EMPTY => { break; },
                Cell::BOX   => {
                    i += 1;
                    continue;
                },
                Cell::WALL  => { continue 'outer; },
            }
        }

        pos = (x + dx, y + dy);

        if i > 1 {
            map[(y + dy) as usize][(x + dx) as usize] = Cell::EMPTY;
            map[(y + i * dy) as usize][(x + i * dx) as usize] = Cell::BOX;
        }
    }

    let mut gps_sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            match map[row as usize][col as usize] {
                Cell::BOX => {
                    gps_sum += 100 * row + col;
                },
                _ => {},
            }
        }
    }

    println!("{}", gps_sum);
}
