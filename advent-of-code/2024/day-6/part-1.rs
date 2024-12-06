use std::io::{self, BufRead};
use std::collections::HashSet;

enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn main() {
    let stdin = io::stdin();

    // Read Initial State
    let mut obstacles: HashSet<(i64, i64)> = HashSet::new();
    let mut guard_pos: (i64, i64) = (0, 0);
    let mut guard_dir: Direction = Direction::UP;
    let rows: i64;
    let mut cols: i64 = 0;
    {
        let mut row = 0;
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            cols = line.len() as i64;
            for (col, char) in line.char_indices() {
                if char == '#' {
                    obstacles.insert((row, col as i64));
                } else if char == '^' {
                    guard_pos = (row, col as i64);
                }
            }
            row += 1;
        }
        rows = row;
    }
    let obstacles = obstacles;
    let rows = rows;
    let cols = cols;

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    while guard_pos.0 >= 0 && guard_pos.0 < cols && guard_pos.1 >= 0 && guard_pos.1 < rows {
        visited.insert(guard_pos);

        let next_position = match guard_dir {
            Direction::UP    => (guard_pos.0 - 1, guard_pos.1),
            Direction::DOWN  => (guard_pos.0 + 1, guard_pos.1),
            Direction::RIGHT => (guard_pos.0, guard_pos.1 + 1),
            Direction::LEFT  => (guard_pos.0, guard_pos.1 - 1),
        };

        if obstacles.contains(&next_position) {
            guard_dir = match guard_dir {
                Direction::UP    => Direction::RIGHT,
                Direction::DOWN  => Direction::LEFT,
                Direction::RIGHT => Direction::DOWN,
                Direction::LEFT  => Direction::UP,
            };
            continue;
        }

        guard_pos = next_position;
    }

    println!("{}", visited.len());
}
