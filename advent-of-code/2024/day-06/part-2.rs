use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone)]
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
    let mut prev_states: HashSet<(i64, i64, Direction)> = HashSet::new();
    let mut potential: HashSet<(i64, i64)> = HashSet::new();

    while within_grid(&guard_pos, rows, cols) {
        visited.insert(guard_pos);
        prev_states.insert((guard_pos.0, guard_pos.1, guard_dir.clone()));

        let next_pos = get_next_pos(&guard_pos, &guard_dir); 
        let next_dir = get_next_dir(&guard_dir); 

        if obstacles.contains(&next_pos) {
            guard_dir = next_dir;
            continue;
        }

        if !visited.contains(&next_pos) && within_grid(&next_pos, rows, cols) {
            let mut pos = guard_pos;
            let mut dir = next_dir;
            let mut states: HashSet<(i64, i64, Direction)> = HashSet::new();

            while within_grid(&pos, rows, cols) {
                if prev_states.contains(&(pos.0, pos.1, dir.clone())) {
                    potential.insert(next_pos);
                    break;
                }
                if states.contains(&(pos.0, pos.1, dir.clone())) {
                    potential.insert(next_pos);
                    break;
                }

                states.insert((pos.0, pos.1, dir.clone()));

                let npos = get_next_pos(&pos, &dir); 
                let ndir = get_next_dir(&dir); 

                if obstacles.contains(&npos) || npos == next_pos {
                    dir = ndir;
                    continue;
                }
                pos = npos;
            }
        }

        guard_pos = next_pos;
    }

    println!("{}", potential.len());
}

fn get_next_pos(pos: &(i64, i64), dir: &Direction) -> (i64, i64) {
    return match dir {
        Direction::UP    => (pos.0 - 1, pos.1),
        Direction::DOWN  => (pos.0 + 1, pos.1),
        Direction::RIGHT => (pos.0, pos.1 + 1),
        Direction::LEFT  => (pos.0, pos.1 - 1),
    };
}

fn get_next_dir(dir: &Direction) -> Direction {
    return match dir {
        Direction::UP    => Direction::RIGHT,
        Direction::DOWN  => Direction::LEFT,
        Direction::RIGHT => Direction::DOWN,
        Direction::LEFT  => Direction::UP,
    };
}

fn within_grid(pos: &(i64, i64), rows: i64, cols: i64) -> bool {
    return pos.0 >= 0 && pos.0 < cols && pos.1 >= 0 && pos.1 < rows;
}
