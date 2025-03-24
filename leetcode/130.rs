// LeetCode/130 - Surrounded Regions

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();

        let mut safe = vec![vec![false; cols]; rows];
        let mut stack = vec![];

        for col in 0..cols {
            if board[0][col] == 'O' {
                safe[0][col] = true;
                stack.push((0, col));
            }
            if board[rows - 1][col] == 'O' {
                safe[rows - 1][col] = true;
                stack.push((rows - 1, col));
            }
        }
        for row in 1..(rows - 1) {
            if board[row][0] == 'O' {
                safe[row][0] = true;
                stack.push((row, 0));
            }
            if board[row][cols - 1] == 'O' {
                safe[row][cols - 1] = true;
                stack.push((row, cols - 1));
            }
        }

        while stack.len() > 0 {
            let (row, col) = stack.pop().unwrap();

            for (delta_row, delta_col) in DIRS {
                let row_next = row as i32 + delta_row;
                if row_next < 0 || row_next >= rows as i32 {
                    continue;
                }
                let col_next = col as i32 + delta_col;
                if col_next < 0 || col_next >= cols as i32 {
                    continue;
                }
                let (row_next, col_next) = (row_next as usize, col_next as usize);
                if board[row_next][col_next] == 'X' {
                    continue;
                }
                if safe[row_next][col_next] {
                    continue;
                }
                safe[row_next][col_next] = true;
                stack.push((row_next, col_next));
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                if !safe[row][col] {
                    board[row][col] = 'X';
                }
            }
        }
    }
}
