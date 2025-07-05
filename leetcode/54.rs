// LeetCode/54 - Spiral Matrix

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        const DIRS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut seen = vec![vec![false; cols]; rows];
        let (mut row, mut col) = (0, 0);
        let mut done = 0;

        let mut dir = 0;

        let mut ans = vec![];

        loop {
            ans.push(matrix[row][col]);
            seen[row][col] = true;
            done += 1;

            if done == rows * cols {
                break;
            }

            loop {
                let new_row = ((row as i32) + DIRS[dir].0) as usize;
                let new_col = ((col as i32) + DIRS[dir].1) as usize;
                if new_row >= 0
                    && new_row < rows
                    && new_col >= 0
                    && new_col < cols
                    && !seen[new_row][new_col]
                {
                    (row, col) = (new_row, new_col);
                    break;
                }
                dir = (dir + 1) % DIRS.len();
            }
        }

        return ans;
    }
}
