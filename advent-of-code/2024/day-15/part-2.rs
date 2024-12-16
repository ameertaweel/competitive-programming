use std::io::{self, BufRead};

const MOVES: [(i64, i64); 4] = [
    ( 0, -1),
    ( 1,  0),
    ( 0,  1),
    (-1,  0),
];

#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
enum Cell {
    EMPTY,
    BOX_BEG,
    BOX_END,
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
                '.' => vec![Cell::EMPTY, Cell::EMPTY],
                'O' => vec![Cell::BOX_BEG, Cell::BOX_END],
                '#' => vec![Cell::WALL, Cell::WALL],
                '@' => {
                    pos = (2 * col as i64, row as i64);
                    vec![Cell::EMPTY, Cell::EMPTY]
                },
                _   => panic!("Shouldn't Happen"),
            }).flatten().collect());
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

    for m in moves.iter() {
        let mut map_clone = map.clone();
        let mut pos_clone = pos.clone();
        let succ = match m {
            Move::UP    => push_ud_person(&MOVES[0], &mut pos_clone, &mut map_clone),
            Move::RIGHT => push_lr(&MOVES[1], &mut pos_clone, &mut map_clone),
            Move::DOWN  => push_ud_person(&MOVES[2], &mut pos_clone, &mut map_clone),
            Move::LEFT  => push_lr(&MOVES[3], &mut pos_clone, &mut map_clone),
        };
        if succ {
            map = map_clone;
            pos = pos_clone;
        }
    }

    let mut gps_sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            match map[row as usize][col as usize] {
                Cell::BOX_BEG => {
                    gps_sum += 100 * row + col;
                },
                _ => {},
            }
        }
    }

    println!("{}", gps_sum);
}

fn push_ud_person(d: &(i64, i64), pos: &mut (i64, i64), map: &mut Vec<Vec<Cell>>) -> bool {
    let x = pos.0;
    let y = pos.1;
    let (dx, dy) = d;

    let target = &map[(y + dy) as usize][(x + dx) as usize];

    match target {
        Cell::WALL  => { return false; },
        Cell::EMPTY => {
            *pos = (x + dx, y + dy);
            return true;
        },
        Cell::BOX_BEG => {
            if push_ud_box(d, (x + dx, y + dy), map){
                *pos = (x + dx, y + dy);
                return true;
            }
            return false;
        },
        Cell::BOX_END => {
            if push_ud_box(d, (x + dx - 1, y + dy), map){
                *pos = (x + dx, y + dy);
                return true;
            }
            return false;
        },
    }
}

fn push_ud_box(d: &(i64, i64), pos: (i64, i64), map: &mut Vec<Vec<Cell>>) -> bool {
    let x = pos.0;
    let y = pos.1;
    let (dx, dy) = d;

    let target_l = map[(y + dy) as usize][(x + dx)     as usize].clone();
    let target_r = map[(y + dy) as usize][(x + dx + 1) as usize].clone();

    if target_l == Cell::WALL || target_r == Cell::WALL {
        return false;
    }

    if target_l == Cell::EMPTY && target_r == Cell::EMPTY {
        map[y as usize][x       as usize] = Cell::EMPTY;
        map[y as usize][(x + 1) as usize] = Cell::EMPTY;
        map[(y + dy) as usize][(x + dx)     as usize] = Cell::BOX_BEG;
        map[(y + dy) as usize][(x + dx + 1) as usize] = Cell::BOX_END;
        return true;
    }

    if target_l == Cell::BOX_BEG {
        if push_ud_box(d, (x + dx, y + dy), map) {
            map[y as usize][x       as usize] = Cell::EMPTY;
            map[y as usize][(x + 1) as usize] = Cell::EMPTY;
            map[(y + dy) as usize][(x + dx)     as usize] = Cell::BOX_BEG;
            map[(y + dy) as usize][(x + dx + 1) as usize] = Cell::BOX_END;
            return true;
        }
        return false;
    }

    let mut succ = true;
    if target_l == Cell::BOX_END {
        succ &= push_ud_box(d, (x + dx - 1, y + dy), map);
    }
    if target_r == Cell::BOX_BEG {
        succ &= push_ud_box(d, (x + dx + 1, y + dy), map);
    }
    
    if succ {
        map[y as usize][x       as usize] = Cell::EMPTY;
        map[y as usize][(x + 1) as usize] = Cell::EMPTY;
        map[(y + dy) as usize][(x + dx)     as usize] = Cell::BOX_BEG;
        map[(y + dy) as usize][(x + dx + 1) as usize] = Cell::BOX_END;
        return true;
    }

    return false;
}

fn push_lr(d: &(i64, i64), pos: &mut (i64, i64), map: &mut Vec<Vec<Cell>>) -> bool {
    let x = pos.0;
    let y = pos.1;
    let (dx, dy) = d;

    let mut i = 1;

    loop {
        let x = x + i * dx;
        let y = y + i * dy;
        let cell = &map[y as usize][x as usize];
        match cell {
            Cell::EMPTY => { break; },
            Cell::WALL  => { return false; },
            _           => {
                i += 1;
                continue;
            },
        }
    }

    *pos = (x + dx, y + dy);

    while i > 1 {
        map[(y + i * dy) as usize][(x + i * dx) as usize] = map[(y + (i - 1) * dy) as usize][(x + (i - 1) * dx) as usize].clone();
        i -= 1;
    }
    map[(y + dy) as usize][(x + dx) as usize] = Cell::EMPTY;

    return true;
}
