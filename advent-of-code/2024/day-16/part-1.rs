use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};

const COST_MOVE: i64 = 1;
const COST_TURN: i64 = 1000;

fn main() {
    let stdin = io::stdin();

    let mut pos_beg: (i64, i64) = (0, 0);
    let mut pos_end: (i64, i64) = (0, 0);
    let mut is_wall: Vec<Vec<bool>> = Vec::new();
    for (row, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();

        is_wall.push(vec![false; line.len()]);

        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => { is_wall[row][col] = true; },
                'S' => { pos_beg = (row as i64, col as i64); },
                'E' => { pos_end = (row as i64, col as i64); },
                 _  => { },
            }
        }
    }

    let pos_beg = pos_beg;
    let pos_end = pos_end;
    let is_wall = is_wall;
    let rows    = is_wall.len() as i64;
    let cols    = is_wall[0].len() as i64;

    // Initialize to an upper bound on minimum cost
    let mut min_cost_path: i64 = rows * cols * (COST_MOVE + COST_TURN);

    // (Row, Col), Direction, Cost So Far
    let mut q: VecDeque<((i64, i64), Direction, i64)> = VecDeque::new();
    q.push_back((pos_beg, Direction::East, 0));

    // ((Row, Col), Direction) -> Minimum Cost So Far
    let mut min_cost_state: HashMap<((i64, i64), Direction), i64> = HashMap::new();

    while q.len() > 0 {
        let ((row, col), dir, cost) = q.pop_front().unwrap();
        let pos = (row, col);

        if !within_grid(&pos, rows, cols) { continue; }

        if is_wall[row as usize][col as usize] { continue; }

        if pos == pos_end {
            if cost < min_cost_path {
                min_cost_path = cost;
            }
            continue;
        }

        if *min_cost_state.get(&(pos, dir.clone())).unwrap_or(&(cost + 1)) <= cost { continue; }

        min_cost_state.insert((pos, dir.clone()), cost);

        let d = dir.clone();
        q.push_back((d.apply(&pos), d, cost + COST_MOVE));
        let d = dir.turn_cw();
        q.push_back((d.apply(&pos), d, cost + COST_MOVE + COST_TURN));
        let d = dir.turn_ccw();
        q.push_back((d.apply(&pos), d, cost + COST_MOVE + COST_TURN));
    }

    println!("{}", min_cost_path);
}

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn offset(&self) -> (i64, i64) {
        match self {
            Direction::North => (-1,  0),
            Direction::East  => ( 0,  1),
            Direction::South => ( 1,  0),
            Direction::West  => ( 0, -1),
        }
    }

    const fn turn_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North,
        }
    }

    const fn turn_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East  => Direction::North,
            Direction::South => Direction::East,
            Direction::West  => Direction::South,
        }
    }

    fn apply(&self, pos: &(i64, i64)) -> (i64, i64) {
        let (row, col) = pos;
        let (delta_row, delta_col) = self.offset();
        (row + delta_row, col + delta_col)
    }
}

fn within_grid(pos: &(i64, i64), rows: i64, cols: i64) -> bool {
    return pos.0 >= 0 && pos.0 < cols && pos.1 >= 0 && pos.1 < rows;
}
